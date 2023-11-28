use sysinfo::{NetworkExt, System, SystemExt};

fn main(){
    // creating a new system instance
    let mut system = System::new_all();

    loop {
        // Refresh all information of the system
        system.refresh_all();

        // Display all disks' information:
        println!("=> Disk information:");
        for disk in system.disks(){
            println!("{:?},", disk);
        }

        // Components temperature:
        println!("\n=> components:");
        for component in system.components() {
            println!("{:?}", component);
        }
        
        println!("\n=> System:");
        // RAM and swap information:
        println!("total memory: {} Mb", system.total_memory()/1000);
        println!("used memory : {} Mb", system.used_memory()/1000);
        println!("total swap  : {} Mb", system.total_swap()/1000);
        println!("used swap   : {} Mb", system.used_swap()/1000);


        // Print general information 
        println!("\n");
        println!("System name       {:?} ",system.name() );
        println!("OS:               {:?}", system.os_version());
        println!("Kernel version:   {:?}", system.kernel_version());
        println!("System host name: {:?} \n", system.host_name());


        // Network speed : name, data received and transmitted
        println!("\n=> Network informations:");
        for (interface_name, data) in system.networks(){
            println!("{} : {}/{} B", interface_name, data.received(), data.transmitted());
        }

        //for (pid, process) in system.processes(){
        //    println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
        //}


        std::thread::sleep(std::time::Duration::from_secs(20));
    }
}