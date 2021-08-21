let irc_commpacketet = match irc_ipc2_rx.try_recv() {
    Result => println!("yep"),
    TryRecvError => println!("Unhandled match")
}
