use std::{path::Path, io::Cursor};

use gh_updater::ReleaseFinderConfig;
use semver::Version;
use zip::ZipArchive;

#[derive(Copy, Clone, PartialEq, Eq)]
enum WhichVersion {
    Current,
    Release,
    Prerelease
}

fn get_version(current: &Version, release: Option<&Version>, prerelease: Option<&Version>) -> WhichVersion {
    if !current.pre.is_empty() {
        if let Some(prerelease) = prerelease {
            if current != prerelease {
                WhichVersion::Prerelease
            } else {
                WhichVersion::Current
            }
        } else {
            WhichVersion::Current
        }
    } else {
        if let Some(release) = release {
            if current != release {
                WhichVersion::Release
            } else {
                WhichVersion::Current
            }
        } else {
            WhichVersion::Current
        }
    }
}

pub fn check_for_updates() {
    if Path::new("sd:/ryujinx.txt").exists() {
        println!("Auto updater exiting as Ryujinx was encountered.");
        return;
    }

    let release = ReleaseFinderConfig::new("HewDraw-Remix")
        .with_author("HDR-Development")
        .with_repository("HewDraw-Remix")
        .with_prereleases(true)
        .find_release();
    
    let (release, prerelease) = match release {
        Ok(r) => r,
        Err(e) => {
            println!("HDR failed to check for updates: {:?}", e);
            return;
        }
    };

    // Get the current version
    let current_version = Version::parse(env!("CARGO_PKG_VERSION").trim_start_matches("v")).unwrap();

    // Get the release version after getting rid of everything before the "v" (SemVer comes after "v")
    let release_version = release
        .as_ref()
        .map(|x| x.get_release_tag().split("v").last())
        .flatten()
        .map(|x| Version::parse(x).ok())
        .flatten();

    // Get the prerelease version after getting rid of everything before the "v" (SemVer comes after "v")
    let prerelease_version = prerelease
        .as_ref()
        .map(|x| x.get_release_tag().split("v").last())
        .flatten()
        .map(|x| Version::parse(x).ok())
        .flatten();

    // Don't bother updating if we are a dev build because it's in development
    if current_version.pre.as_str() == "dev" {
        return;
    }
    
    // get which version to update to and get the asset
    let ver = get_version(&current_version, release_version.as_ref(), prerelease_version.as_ref());

    // prompt user if they want to update
    let should_update = match ver {
        WhichVersion::Prerelease => skyline_web::Dialog::yes_no("A new version of HDR (nightly) was encountered.<br>Do you want to install it?"),
        WhichVersion::Release => skyline_web::Dialog::yes_no("A new version of HDR was encountered.<br>Do you want to install it?"),
        WhichVersion::Current => return,
    };

    if !should_update {
        return;
    }

    let asset = match ver {
        WhichVersion::Prerelease => prerelease.unwrap().get_asset_by_name("hdr-switch.zip"),
        WhichVersion::Release => release.unwrap().get_asset_by_name("hdr-switch.zip"),
        WhichVersion::Current => unreachable!(),
    };

    // get the zip reader on the zip archive to extract to SD root
    let mut zip = match asset {
        Some(asset) => match ZipArchive::new(Cursor::new(asset)) {
            Ok(zip) => zip,
            Err(e) => {
                println!("HDR failed to parse zip data: {:?}", e);
                return;
            }
        },
        None => return
    };

    // try to extract the zip to the SD root and restart the application
    match zip.extract("sd:/") {
        Ok(_) => unsafe { 
            skyline_web::DialogOk::ok("The applicatio will now restart.");
            skyline::nn::oe::RequestToRelaunchApplication();
        },
        Err(e) => {
            panic!("HDR failed to extract update ZIP. Reason: {:?}", e);
        }
    }
}