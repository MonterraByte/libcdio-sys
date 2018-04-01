// Simple program to list track numbers and logical sector numbers of
// a Compact Disc using libcdio.

extern crate libcdio_sys;

use std::ptr;
use std::process;

use libcdio_sys::{lsn_t, track_t, CdIo_t, cdio_track_enums_CDIO_CDROM_LEADOUT_TRACK,
                  driver_id_t_DRIVER_UNKNOWN, CDIO_INVALID_LSN};
use libcdio_sys::{cdio_destroy, cdio_get_disc_last_lsn, cdio_get_first_track_num,
                  cdio_get_num_tracks, cdio_get_track_lsn, cdio_open};

fn main() {
    let p_cdio: *mut CdIo_t = unsafe { cdio_open(ptr::null(), driver_id_t_DRIVER_UNKNOWN) };

    if p_cdio.is_null() {
        println!("Couldn't find a driver.. leaving.");
        process::exit(1);
    }

    println!("Disc last LSN: {}", unsafe {
        cdio_get_disc_last_lsn(p_cdio)
    });

    let i_tracks: track_t = unsafe { cdio_get_num_tracks(p_cdio) };
    let i_first_track: track_t = unsafe { cdio_get_first_track_num(p_cdio) };
    let mut i: i32 = i_first_track as i32;

    println!(
        "CD-ROM Track List ({} - {})",
        i_first_track,
        i_first_track as i32 + i_tracks as i32 - 1
    );

    println!("  #:  LSN");

    for _j in 0..i_tracks {
        let lsn: lsn_t = unsafe { cdio_get_track_lsn(p_cdio, i as u8) };
        if lsn != CDIO_INVALID_LSN {
            println!("{:3}: {:06}", i, lsn as u64);
        }
        i = i + 1;
    }

    println!(
        "{:3X}: {} leadout",
        cdio_track_enums_CDIO_CDROM_LEADOUT_TRACK,
        unsafe { cdio_get_track_lsn(p_cdio, cdio_track_enums_CDIO_CDROM_LEADOUT_TRACK as u8) } as u64
    );

    unsafe { cdio_destroy(p_cdio) };
}
