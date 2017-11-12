extern crate posidonius;
extern crate time;
extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings};
use posidonius::Integrator;
use std::path::Path;


fn main() {
    let t1 = time::precise_time_s();

    let matches = App::new("Posidonius")
                            .version("2017.01.27")
                            .author("Sergi Blanco-Cuaresma - http://www.blancocuaresma.com/s/")
                            .about("N-Body simulator for planetary system affected by tidal effects. Based on Mercury-T from Emeline Bolmont.")
                            .subcommand(SubCommand::with_name("start")
                                    .about("Start a simulation")
                                    .arg(Arg::with_name("start_case_filename")
                                        .required(true)
                                        .index(1)
                                        .help("JSON case description"))
                                    .arg(Arg::with_name("snapshot_filename")
                                        .required(true)
                                        .index(2)
                                        .help("Recovery snapshot filename")) 
                                    .arg(Arg::with_name("historic_snapshot_filename")
                                        .required(true)
                                        .index(3)
                                        .help("Historic snapshot filename")) 
                                    .arg(Arg::with_name("no-verify-integrity")
                                        .short("n")
                                        .long("no-verify-integrity")
                                        .multiple(false)
                                        .help("Do not verify file integrity"))
                                    .arg(Arg::with_name("silent")
                                        .short("s")
                                        .long("silent")
                                        .multiple(false)
                                        .help("Only print INFO/WARNING/ERROR messages"))
                                     )
                            .subcommand(SubCommand::with_name("resume")
                                    .about("Resume a simulation")
                                    .arg(Arg::with_name("resume_case_filename")
                                        .required(true)
                                        .help("Recovery snapshot filename"))
                                    .arg(Arg::with_name("historic_snapshot_filename")
                                        .required(true)
                                        .help("Historic snapshot filename")) 
                                    .arg(Arg::with_name("no-verify-integrity")
                                        .short("n")
                                        .long("no-verify-integrity")
                                        .multiple(false)
                                        .help("Do not verify file integrity"))
                                    .arg(Arg::with_name("silent")
                                        .short("s")
                                        .long("silent")
                                        .multiple(false)
                                        .help("Only print INFO/WARNING/ERROR messages"))
                                    )
                            .setting(AppSettings::SubcommandRequiredElseHelp)
                          .get_matches();

    let first_universe_integrator_snapshot_filename;
    let universe_integrator_snapshot_filename;
    let universe_history_filename;
    let verify_integrity;
    let silent_mode;
    let resume;

    match matches.subcommand() {
        ("start", Some(start_matches)) =>{
            first_universe_integrator_snapshot_filename = start_matches.value_of("start_case_filename").unwrap();
            universe_integrator_snapshot_filename = start_matches.value_of("snapshot_filename").unwrap();
            universe_history_filename = start_matches.value_of("historic_snapshot_filename").unwrap();
            silent_mode = start_matches.is_present("silent");
            verify_integrity = start_matches.is_present("no-verify-integrity");
            resume = false;
        },
        ("resume", Some(resume_matches)) =>{
            universe_integrator_snapshot_filename = resume_matches.value_of("resume_case_filename").unwrap();
            first_universe_integrator_snapshot_filename = &universe_integrator_snapshot_filename;
            universe_history_filename = resume_matches.value_of("historic_snapshot_filename").unwrap();
            silent_mode = resume_matches.is_present("silent");
            verify_integrity = ! resume_matches.is_present("no-verify-integrity");
            resume = true;
        },
        ("", None)   => unreachable!(),
        _            => unreachable!(),
    }

    let universe_integrator_snapshot_path = Path::new(&universe_integrator_snapshot_filename);

    let first_universe_integrator_snapshot_path = Path::new(&first_universe_integrator_snapshot_filename);
    
    // Start/Resume from snapshot
    let mut universe_integrator = match posidonius::WHFastHelio::restore_snapshot(&first_universe_integrator_snapshot_path, verify_integrity) {
        Ok(restored_case) => { restored_case },
        Err(_) => { 
            if resume {
                panic!("[PANIC {} UTC] It was not possible to resume the simulation", time::now_utc().strftime("%Y.%m.%d %H:%M:%S").unwrap());
            } else {
                panic!("[PANIC {} UTC] It was not possible to start the simulation", time::now_utc().strftime("%Y.%m.%d %H:%M:%S").unwrap());
            }
        },
    };

    //// Other integrators:
    //let mut universe_integrator = posidonius::Ias15::restore_snapshot(&first_universe_integrator_snapshot_path).unwrap();
    //let mut universe_integrator = posidonius::LeapFrog::restore_snapshot(&first_universe_integrator_snapshot_path).unwrap();

    // Create/recover historic snapshot
    let expected_n_bytes = (universe_integrator.n_historic_snapshots as u64) 
                                * posidonius::output::n_bytes_per_particle_in_historic_snapshot()
                                * (universe_integrator.universe.n_particles as u64);
    let universe_history_path = Path::new(&universe_history_filename);

    if !resume && universe_integrator_snapshot_path.exists() {
        panic!("[PANIC {} UTC] File '{}' already exists.", time::now_utc().strftime("%Y.%m.%d %H:%M:%S").unwrap(), universe_integrator_snapshot_filename);
    } else if !resume && universe_history_path.exists() {
        panic!("[PANIC {} UTC] File '{}' already exists.", time::now_utc().strftime("%Y.%m.%d %H:%M:%S").unwrap(), universe_history_filename);
    }

    let mut universe_history_writer = posidonius::output::get_universe_history_writer(universe_history_path, expected_n_bytes);

    // Simulate
    loop {
        match universe_integrator.iterate(&mut universe_history_writer, silent_mode) {
            Ok(recovery_snapshot_time_trigger) => {
                if recovery_snapshot_time_trigger {
                    // Save a universe snapshot so that we can resume in case of failure
                    universe_integrator.prepare_for_recovery_snapshot(&mut universe_history_writer);
                    posidonius::output::write_recovery_snapshot(&universe_integrator_snapshot_path, &universe_integrator);
                }
            },
            Err(e) => { println!("[ERROR {} UTC] {}", time::now_utc().strftime("%Y.%m.%d %H:%M:%S").unwrap(), e); break; }
        };
    }

    let t2 = time::precise_time_s();
    let d = t2 - t1;
    println!("[INFO {} UTC] Execution time: {} seconds", time::now_utc().strftime("%Y.%m.%d %H:%M:%S").unwrap(), d);
}

