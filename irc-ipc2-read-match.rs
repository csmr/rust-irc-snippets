let irc_commpacket = match irc_ipc2_rx.try_recv() {
    Ok(irc_commpacket_rx) => {
        println!("Event recieved: {}",irc_commpacket_rx.event);
    }
    Err(error) => {
        println!("Error caught: {}",error);
    }
};
thread::sleep(_DURATION_1000MS);
