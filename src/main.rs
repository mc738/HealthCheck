use sysinfo::{ProcessExt, SystemExt};

fn main() {

    let mut sys = sysinfo::System::new_all();

    sys.refresh_all();
    

    for (pid, proc_) in sys.get_processes() {
        println!("{}:{} => {:?}", pid, proc_.name(), proc_.status());
    }


    for component in sys.get_components() {
        println!("{:?}", component);
    }

    for disk in sys.get_disks() {
        println!("{:?}", disk);
    }

    println!("total memory: {} KiB", sys.get_total_memory());
    println!("used memory : {} KiB", sys.get_used_memory());
    println!("total swap  : {} KiB", sys.get_total_swap());
    println!("used swap   : {} KiB", sys.get_used_swap());

}
