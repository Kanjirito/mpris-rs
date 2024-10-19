use std::collections::HashMap;

use futures_util::join;
use zbus::{names::BusName, Connection};

use crate::{
    metadata::MetadataValue,
    proxies::{MediaPlayer2Proxy, PlayerProxy},
    LoopStatus, Metadata, Mpris, MprisDuration, MprisError, PlaybackStatus, TrackID, MPRIS2_PREFIX,
};

pub struct Player {
    bus_name: BusName<'static>,
    mp2_proxy: MediaPlayer2Proxy<'static>,
    player_proxy: PlayerProxy<'static>,
}

impl Player {
    pub async fn new(mpris: &Mpris, bus_name: BusName<'static>) -> Result<Player, MprisError> {
        Player::new_from_connection(mpris.connection.clone(), bus_name).await
    }

    pub(crate) async fn new_from_connection(
        connection: Connection,
        bus_name: BusName<'static>,
    ) -> Result<Player, MprisError> {
        let (mp2_proxy, player_proxy) = join!(
            MediaPlayer2Proxy::new(&connection, bus_name.clone()),
            PlayerProxy::new(&connection, bus_name.clone())
        );
        Ok(Player {
            bus_name,
            mp2_proxy: mp2_proxy?,
            player_proxy: player_proxy?,
        })
    }

    pub async fn metadata(&self) -> Result<Metadata, MprisError> {
        Ok(self.raw_metadata().await?.try_into()?)
    }

    pub async fn raw_metadata(&self) -> Result<HashMap<String, MetadataValue>, MprisError> {
        let data = self.player_proxy.metadata().await?;
        let raw: HashMap<String, MetadataValue> =
            data.into_iter().map(|(k, v)| (k, v.into())).collect();
        Ok(raw)
    }

    pub async fn is_running(&self) -> Result<bool, MprisError> {
        match self.mp2_proxy.ping().await {
            Ok(_) => Ok(true),
            Err(e) => {
                if let zbus::Error::MethodError(ref err_name, _, _) = e {
                    if err_name.as_str() == "org.freedesktop.DBus.Error.ServiceUnknown" {
                        Ok(false)
                    } else {
                        Err(e.into())
                    }
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub fn bus_name(&self) -> &str {
        self.bus_name.as_str()
    }

    pub fn bus_name_trimmed(&self) -> &str {
        self.bus_name().trim_start_matches(MPRIS2_PREFIX)
    }

    pub async fn quit(&self) -> Result<(), MprisError> {
        Ok(self.mp2_proxy.quit().await?)
    }

    pub async fn can_quit(&self) -> Result<bool, MprisError> {
        Ok(self.mp2_proxy.can_quit().await?)
    }

    pub async fn raise(&self) -> Result<(), MprisError> {
        Ok(self.mp2_proxy.raise().await?)
    }

    pub async fn can_raise(&self) -> Result<bool, MprisError> {
        Ok(self.mp2_proxy.can_raise().await?)
    }

    pub async fn desktop_entry(&self) -> Result<String, MprisError> {
        Ok(self.mp2_proxy.desktop_entry().await?)
    }

    pub async fn supports_track_list(&self) -> Result<bool, MprisError> {
        Ok(self.mp2_proxy.has_track_list().await?)
    }

    pub async fn identity(&self) -> Result<String, MprisError> {
        Ok(self.mp2_proxy.identity().await?)
    }

    pub async fn supported_mime_types(&self) -> Result<Vec<String>, MprisError> {
        Ok(self.mp2_proxy.supported_mime_types().await?)
    }

    pub async fn supported_uri_schemes(&self) -> Result<Vec<String>, MprisError> {
        Ok(self.mp2_proxy.supported_uri_schemes().await?)
    }

    pub async fn can_control(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.can_control().await?)
    }

    pub async fn next(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.next().await?)
    }

    pub async fn can_go_next(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.can_go_next().await?)
    }

    pub async fn previous(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.previous().await?)
    }

    pub async fn can_go_previous(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.can_go_previous().await?)
    }

    pub async fn play(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.play().await?)
    }

    pub async fn can_play(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.can_play().await?)
    }

    pub async fn pause(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.pause().await?)
    }

    pub async fn can_pause(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.can_pause().await?)
    }

    pub async fn play_pause(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.play_pause().await?)
    }

    pub async fn stop(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.stop().await?)
    }

    pub async fn stop_after_current(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.stop_after_current().await?)
    }

    pub async fn seek(&self, offset_in_microseconds: i64) -> Result<(), MprisError> {
        Ok(self.player_proxy.seek(offset_in_microseconds).await?)
    }

    pub async fn seek_forwards(&self, offset: MprisDuration) -> Result<(), MprisError> {
        Ok(self.player_proxy.seek(offset.into()).await?)
    }

    pub async fn seek_backwards(&self, offset: MprisDuration) -> Result<(), MprisError> {
        Ok(self.player_proxy.seek(-i64::from(offset)).await?)
    }

    pub async fn can_seek(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.can_seek().await?)
    }

    pub async fn get_position(&self) -> Result<MprisDuration, MprisError> {
        Ok(self.player_proxy.position().await?.try_into()?)
    }

    pub async fn set_position(
        &self,
        track_id: &TrackID,
        position: MprisDuration,
    ) -> Result<(), MprisError> {
        Ok(self
            .player_proxy
            .set_position(&track_id.get_object_path(), position.into())
            .await?)
    }

    pub async fn get_loop_status(&self) -> Result<LoopStatus, MprisError> {
        Ok(self.player_proxy.loop_status().await?.parse()?)
    }

    pub async fn set_loop_status(&self, loop_status: LoopStatus) -> Result<(), MprisError> {
        Ok(self
            .player_proxy
            .set_loop_status(loop_status.as_str())
            .await?)
    }

    pub async fn playback_status(&self) -> Result<PlaybackStatus, MprisError> {
        Ok(self.player_proxy.playback_status().await?.parse()?)
    }

    pub async fn open_uri(&self, uri: &str) -> Result<(), MprisError> {
        Ok(self.player_proxy.open_uri(uri).await?)
    }

    pub async fn maximum_rate(&self) -> Result<f64, MprisError> {
        Ok(self.player_proxy.maximum_rate().await?)
    }

    pub async fn minimum_rate(&self) -> Result<f64, MprisError> {
        Ok(self.player_proxy.minimum_rate().await?)
    }

    pub async fn get_playback_rate(&self) -> Result<f64, MprisError> {
        Ok(self.player_proxy.rate().await?)
    }

    pub async fn set_playback_rate(&self, rate: f64) -> Result<(), MprisError> {
        Ok(self.player_proxy.set_rate(rate).await?)
    }

    pub async fn get_shuffle(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.shuffle().await?)
    }

    pub async fn set_shuffle(&self, shuffle: bool) -> Result<(), MprisError> {
        Ok(self.player_proxy.set_shuffle(shuffle).await?)
    }

    pub async fn get_volume(&self) -> Result<f64, MprisError> {
        Ok(self.player_proxy.volume().await?)
    }

    pub async fn set_volume(&self, volume: f64) -> Result<(), MprisError> {
        Ok(self.player_proxy.set_volume(volume).await?)
    }
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("bus_name", &self.bus_name())
            .finish()
    }
}
