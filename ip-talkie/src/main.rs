use clap::{Arg, Command};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{StreamConfig};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

// Helper function to get a default audio format
fn get_default_format() -> cpal::SupportedStreamConfig {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Failed to get default output device");
    device.default_output_config().expect("Failed to get default output config")
}

// RTP Packet structure
#[derive(Debug)]
struct RtpPacket {
    payload: Vec<u8>,
}

// Serialize RTP packet
fn serialize_rtp_packet(packet: &RtpPacket) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&packet.payload);
    data
}

// Deserialize RTP packet
fn deserialize_rtp_packet(data: &[u8]) -> RtpPacket {
    RtpPacket {
        payload: data.to_vec(),
    }
}

// Main function to run the VoIP application
fn main() {
    let matches = Command::new("VoIP Walkie-Talkie")
        .version("1.0")
        .about("Peer-to-peer voice chat application")
        .arg(Arg::new("host")
            .short('h')
            .long("host")
            .value_name("HOST")
            .help("The IP address of the friend to connect to")
            .required(true))
        .arg(Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("The port number of the friend to connect to")
            .required(true))
        .get_matches();

    let friend_ip = matches.get_one::<String>("host").unwrap().as_str();
    let friend_port: u16 = matches.get_one::<String>("port").unwrap().parse().expect("Invalid port number");

    let rtp_port = 49170;  // Common RTP port for both sending and receiving

    let local_socket = Arc::new(UdpSocket::bind(("0.0.0.0", rtp_port)).expect("Failed to bind UDP socket"));
    let remote_addr: SocketAddr = format!("{}:{}", friend_ip, friend_port).parse().expect("Invalid socket address");

    let audio_format = get_default_format();
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("Failed to get input device");
    let output_device = host.default_output_device().expect("Failed to get output device");

    let config: StreamConfig = audio_format.config().clone();

    // Create a flag to indicate if the application is running
    let running = Arc::new(AtomicBool::new(true));

    // Set up signal handling for graceful shutdown
    {
        let running_clone = Arc::clone(&running);
        ctrlc::set_handler(move || {
            println!("Received Ctrl+C! Shutting down...");
            running_clone.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }

    // Clone the socket and running flag for sending and receiving threads
    let socket_clone_send = Arc::clone(&local_socket);
    let socket_clone_recv = Arc::clone(&local_socket);

    // Thread to capture and send audio
    let send_thread = {
        let running_send = Arc::clone(&running);
        let moved_running_send = Arc::clone(&running);
        let config_send = config.clone();
        thread::spawn(move || {
            let err_fn = |err| eprintln!("Error in input stream: {}", err);
            let stream = input_device.build_input_stream(
                &config_send,
                move |data: &[f32], _| {
                    if !moved_running_send.load(Ordering::SeqCst) {
                        println!("Stopping send thread...");
                        return;
                    }
                    let rtp_packet = RtpPacket {
                        payload: data.iter().map(|&sample| (sample * 32767.0) as u8).collect(),
                    };
                    let rtp_data = serialize_rtp_packet(&rtp_packet);
                    socket_clone_send.send_to(&rtp_data, remote_addr).expect("Failed to send data");
                },
                err_fn,
                None::<Duration>,  // Providing the fourth argument as None
            ).unwrap();
            stream.play().unwrap();
            while running_send.load(Ordering::SeqCst) {
                thread::sleep(std::time::Duration::from_millis(100));
            }
        })
    };

    // Thread to receive and play audio
    let recv_thread = {
        let running_recv = Arc::clone(&running);
        let moved_running_recv = Arc::clone(&running);
        let config_recv = config.clone();
        thread::spawn(move || {
            let err_fn = |err| eprintln!("Error in output stream: {}", err);
            let stream = output_device.build_output_stream(
                &config_recv,
                move |data: &mut [f32], _| {
                    if !moved_running_recv.load(Ordering::SeqCst) {
                        println!("Stopping receive thread...");
                        return;
                    }
                    let mut buffer = [0; 1024];
                    if let Ok((size, _)) = socket_clone_recv.recv_from(&mut buffer) {
                        let rtp_packet = deserialize_rtp_packet(&buffer[..size]);
                        for (i, sample) in rtp_packet.payload.iter().enumerate() {
                            if i < data.len() {
                                data[i] = *sample as f32 / 32767.0;
                            }
                        }
                    }
                },
                err_fn,
                None::<Duration>,  // Providing the fourth argument as None
            ).unwrap();
            stream.play().unwrap();
            while running_recv.load(Ordering::SeqCst) {
                thread::sleep(std::time::Duration::from_millis(100));
            }
        })
    };

    // Main loop to keep the application running until interrupted
    while running.load(Ordering::SeqCst) {
        thread::sleep(std::time::Duration::from_millis(100));
    }

    send_thread.join().expect("Failed to join send thread");
    recv_thread.join().expect("Failed to join receive thread");

    println!("Application has exited gracefully.");
}
