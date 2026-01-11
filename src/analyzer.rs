use url::Url;
use serde::{Deserialize, Serialize};
use crate::error::{BlobdlError, BlobResult};

#[derive(Debug, PartialOrd, PartialEq, Clone, Deserialize, Serialize)]
pub enum DownloadOption {
    /// If the url refers to a video in a playlist and the user only wants to download the single video, YtVideo's value is the video's index in the playlist
    YtVideo(usize),
    YtPlaylist,
}

/// Analyzes the url provided by the user and deduces whether it
/// refers to a youtube video or playlist
pub fn analyze_url(command_line_url: &str) -> BlobResult<DownloadOption> {
    if let Ok(url) = Url::parse(command_line_url) {
        if let Some(domain_name) = url.domain() {
            // All youtube-related urls have "youtu" in them
            if domain_name.contains("youtu") {
                inspect_yt_url(url)
            } else {
                // The url isn't from youtube
                Err(BlobdlError::UnsupportedWebsite)
            }
        } else {
            Err(BlobdlError::DomainNotFound)
        }
    } else {
        Err(BlobdlError::UrlParsingError)
    }
}

/// Given a youtube url determines whether it refers to a video/playlist
fn inspect_yt_url(yt_url: Url) -> BlobResult<DownloadOption> {
    if let Some(query) = yt_url.query() {
        // Also urls can be part of a playlist but not have an index, just an id
        // example: https://www.youtube.com/watch?v=GNxZ_izoC8I&list=PLl-vhnGPY7cqQ0b_NXy1qyMVsA9LHiPmv
        // maybe support for this will be added in the future
        if query.contains("&index="){
            // This video is part of a youtube playlist, but for MCP usage we default to single video
            // Calculate the index and treat as single video
            let index = if let Some(index_location) = query.find("&index=") {
                let slice = &query[index_location + "&index=".len() ..];

                if let Some(second_ampersand_location) = slice.find('&') {
                    // There are url parameters after &index=..
                     &slice[..second_ampersand_location]
                } else {
                    slice
                }
            } else {
                return Err(BlobdlError::PlaylistUrlError);
            };

            if let Ok(parsed) = index.parse() {
                return Ok(DownloadOption::YtVideo(parsed));
            } else {
                return Err(BlobdlError::UrlIndexParsingError);
            }
        }
        
        // Only treat as playlist if the path explicitly contains "playlist"
        // URLs with "list=" parameter but "v=" parameter should be treated as single video
        // This is useful for MCP usage where interactive prompts are not desired
        if yt_url.path().contains("playlist") {
            return Ok(DownloadOption::YtPlaylist);
        }

        // This url is probably referring to a video or a short
        return Ok(DownloadOption::YtVideo(1));
    }

    Err(BlobdlError::QueryCouldNotBeParsed)
}