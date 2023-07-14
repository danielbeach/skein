use skein::Scheduler;

fn main() {
    let lord = Scheduler::new("127.0.0.1:1024".to_string(), "Lord".to_string());
    lord.run();
}
