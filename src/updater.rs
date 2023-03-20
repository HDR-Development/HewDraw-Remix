use std::{path::Path, io::Cursor};

use gh_updater::ReleaseFinderConfig;
use semver::Version;
use zip::ZipArchive;
use super::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum WhichVersion {
    Current,
    Release,
    Prerelease
}

fn get_version(current: &Version, release: Option<&Version>, prerelease: Option<&Version>) -> WhichVersion {
    if current.pre.as_str() == "nightly" {
        println!("Currently on nightly.");
        if let Some(prerelease) = prerelease {
            if current < prerelease {
                WhichVersion::Prerelease
            } else {
                WhichVersion::Current
            }
        } else {
            WhichVersion::Current
        }
    } else if current.pre.as_str() == "beta" {
        println!("Currently on beta.");
        if let Some(release) = release {
            if current < release {
                WhichVersion::Release
            } else {
                WhichVersion::Current
            }
        } else {
            WhichVersion::Current
        }
    } else {
        WhichVersion::Current
    }
}

pub fn check_for_updates() {
    if is_on_ryujinx() {
        return
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

    match release.as_ref() {
      Some(i) => println!("Found a release!"),
      _ => println!("No release.")
    }

    match prerelease.as_ref() {
        Some(i) => println!("Found a prerelease!"),
        _ => println!("No prerelease.")
      }

    println!("Current version: {}", env!("CARGO_PKG_VERSION"));

    // Get the current version
    let current_version = Version::parse(env!("CARGO_PKG_VERSION").trim_start_matches("v")).unwrap();

    // Don't update dev or release builds
    if current_version.pre.as_str() == "dev" || current_version.pre.is_empty() {
        println!("Not updating, because dev or release build.");
        return;
    }

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

    if let Some(prerelease_version) = prerelease_version.as_ref() {
        println!("Found a prerelease: {}", prerelease_version);
    } else {
        println!("Did not find a prerelease.");
    }

    if let Some(release_version) = release_version.as_ref() {
        println!("Found a release: {}", release_version);
    } else {
        println!("Did not find a release.");
    }

    // get which version to update to and get the asset
    let ver = get_version(&current_version, release_version.as_ref(), prerelease_version.as_ref());

    match ver {
        WhichVersion::Prerelease => println!("We want the prerelease."),
        WhichVersion::Release => println!("We want the release."),
        WhichVersion::Current => println!("We want to stay on the current version."),
    }

    // prompt user if they want to update
    let should_update = match ver {
        WhichVersion::Prerelease => skyline_web::Dialog::yes_no("A new version of HDR (nightly) was encountered.<br>Do you want to install it?"),
        WhichVersion::Release => skyline_web::Dialog::yes_no("A new version of HDR (beta) was encountered.<br>Do you want to install it?"),
        WhichVersion::Current => { println!("Don't need to update."); return },
    };

    // return if we shouldnt update
    if !should_update {
        return;
    }

    println!("downloading the build...");

    let asset = match ver {
        WhichVersion::Prerelease => prerelease.unwrap().get_asset_by_name("hdr-switch.zip"),
        WhichVersion::Release => release.unwrap().get_asset_by_name("hdr-switch.zip"),
        WhichVersion::Current => unreachable!(),
    };

    println!("building zip reader...");

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

    println!("Unzipping data...");

    // try to extract the zip to the SD root and restart the application
    match zip.extract("sd:/") {
        Ok(_) => unsafe { 
            skyline_web::DialogOk::ok("The application will now restart.");
            skyline::nn::oe::RequestToRelaunchApplication();
        },
        Err(e) => {
            panic!("HDR failed to extract update ZIP. Reason: {:?}", e);
        }
    }
}