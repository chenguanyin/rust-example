pub mod hosting {
    pub fn add_to_waitlist() {
        println!("Adding to waitlist")
    }
    pub fn seat_at_table() {
        println!("Seating at table")
    }
}
mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}
