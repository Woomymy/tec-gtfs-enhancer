//! Utility to merge BMC and LETEC feeds to get best of both feeds

mod transforms;
mod util;
use sloggrs::info;
use std::path::PathBuf;

use clap::Parser;

use crate::{
    transforms::util::{
        compute_route_text_color, import_headsigns_from_bmc, import_wheelchair_boarding_from_bmc,
        update_feed_info,
    },
    util::{create_zip_file, extract_zip_file},
};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 'b', long)]
    pub gtfs_bmc: PathBuf,
    #[arg(short = 't', long)]
    pub gtfs_tec: PathBuf,
    #[arg(short, long)]
    pub output_file: PathBuf,
    #[arg(short = 'd', long)]
    pub output_dir: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let bmc_path = args.output_dir.join("bmc");
    let tec_path = args.output_dir.join("tec");

    extract_zip_file(&args.gtfs_bmc, &bmc_path).expect("Can't extract BMC GTFS file");
    extract_zip_file(&args.gtfs_tec, &tec_path).expect("Can't extract TEC GTFS file");

    info!("Importing headsigns");
    import_headsigns_from_bmc(&bmc_path.join("trips.txt"), &tec_path.join("trips.txt"))
        .expect("Can't import heasigns from BMC");

    info!("Importing wheelchair boarding infos");
    import_wheelchair_boarding_from_bmc(&bmc_path.join("stops.txt"), &tec_path.join("stops.txt"))
        .expect("Can't import wheelchair_boarding from BMC");

    info!("Adding route text colors");
    compute_route_text_color(&tec_path.join("routes.txt")).expect("Can't compute route_text_color");

    info!("Updating feed info");
    update_feed_info(
        &bmc_path.join("feed_info.txt"),
        &tec_path.join("feed_info.txt"),
    )
    .expect("Failed to update feed info");

    create_zip_file(&tec_path, &args.output_file).expect("Can't create output zip");
}
