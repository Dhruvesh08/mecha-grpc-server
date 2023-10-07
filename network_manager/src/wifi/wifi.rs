use crate::wifi::errors::{WifiError, WifiErrorCodes};
use anyhow::{bail, Result};
use std::process::Command;
use tracing::{error as trace_error, info, trace};
use wifi_ctrl::sta::{self, NetworkResult, ScanResult};

pub struct WifiModule;

impl WifiModule {
    pub fn new() -> Self {
        trace!(task = "wifi instance", "init");
        Self
    }

    pub fn wifi_status() -> bool {
        trace!(task = "wifi_status", "checking wifi status");
        let output = Command::new("ifconfig")
            .output()
            .expect("Failed to execute ifconfig command");

        info!(task = "wifi_status", "stdout: {:?}", output.stdout);
        let stdout = String::from_utf8(output.stdout).expect("Failed to convert stdout to string");

        // Check if the stdout contains "wlan0"
        stdout.contains("wlan0")
    }

    pub async fn scan_wireless_network(&self) -> Result<Vec<ScanResult>> {
        trace!(task = "scan_wireless_network", "init");
        let mut setup = match sta::WifiSetup::new() {
            Ok(setup) => {
                info!(take = "wifi_setup", "wifi setup successful");
                setup
            }
            Err(e) => {
                trace_error!(
                    task = "scan_wireless_network",
                    "unable to get wifi device status: {}",
                    e
                );
                bail!(WifiError::new(
                    WifiErrorCodes::UnableToGetWifiDeviceStatus,
                    format!("unable to get wifi device status: {}", e),
                ))
            }
        };

        let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
        setup.set_socket_path(proposed_path);

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let (_runtime, wifi_list, _broadcast) = tokio::join!(
            async move {
                if let Err(e) = runtime.run().await {
                    trace_error!(task = "scan_wireless_network", "error: {}", e);
                }
            },
            WifiModule::wifi_list(requester),
            WifiModule::broadcast_listener(broadcast),
        );

        //use wifi_list to get the list of all the wifi networks or else return an error with matching error code
        let wifi_list = match wifi_list {
            Ok(wifi_list) => {
                info!(task = "scan_wireless_network", "wifi list: {:?}", wifi_list);
                wifi_list
            }
            Err(e) => {
                trace_error!(
                    task = "scan_wireless_network",
                    "unable to get wifi device status: {}",
                    e
                );
                bail!(WifiError::new(
                    WifiErrorCodes::UnableToGetWifiDeviceStatus,
                    format!("unable to get wifi device status: {}", e),
                ))
            }
        };

        Ok(wifi_list)
    }

    async fn wifi_list(requester: sta::RequestClient) -> Result<Vec<ScanResult>> {
        trace!(task = "wifi_list", "requesting scan");
        let scan = requester.get_scan().await?;
        requester.shutdown().await?;
        Ok(scan.to_vec())
    }

    pub async fn get_known_wifi_list() -> Result<Vec<NetworkResult>> {
        trace!(task = "get_known_wifi_list", "starting wifi connection");
        let mut setup = match sta::WifiSetup::new() {
            Ok(setup) => {
                info!(task = "wifi_setup", "wifi setup successful");
                setup
            }
            Err(e) => {
                trace_error!(
                    task = "get_known_wifi_list",
                    "unable to get wifi device status: {}",
                    e
                );
                bail!(WifiError::new(
                    WifiErrorCodes::UnableToGetWifiDeviceStatus,
                    format!("unable to get wifi device status: {}", e),
                ))
            }
        };
        let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
        setup.set_socket_path(proposed_path);

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let (_runtime, known_wifi, _broadcast) = tokio::join!(
            async move {
                if let Err(e) = runtime.run().await {
                    trace_error!(task = "get_known_wifi_list", "error: {}", e);
                }
            },
            WifiModule::known_wifi(requester),
            WifiModule::broadcast_listener(broadcast),
        );

        //use known_wifi to get the list of all the known wifi networks or else return an error with matching error code
        let wifi_list = match known_wifi {
            Ok(wifi_list) => {
                info!(task = "get_known_wifi_list", "wifi list: {:?}", wifi_list);
                wifi_list
            }
            Err(e) => {
                trace_error!(
                    task = "get_known_wifi_list",
                    "unable to get wifi device status: {}",
                    e
                );
                bail!(WifiError::new(
                    WifiErrorCodes::UnableToGetWifiDeviceStatus,
                    format!("unable to get wifi device status: {}", e),
                ))
            }
        };

        Ok(wifi_list)
    }

    async fn known_wifi(requester: sta::RequestClient) -> Result<Vec<NetworkResult>> {
        trace!(task = "known_wifi", "requesting networks");
        let scan = requester.get_networks().await?;
        requester.shutdown().await?;
        Ok(scan)
    }

    // we need to write function that return the currnet wifi network name if it is connected to wifi network or else none, how we're going to do that is we use get_known_wifi_list function to get the list of all the known wifi networks and from that reult we can filter the list that has  "flags": "[CURRENT]" and return the ssid of that network or else return none
    pub async fn current_wifi_network(&self) -> Result<ScanResult> {
        let known_wifi_list = WifiModule::get_known_wifi_list().await?;
        let current_wifi = known_wifi_list.iter().find(|&x| x.flags == "[CURRENT]");

        //take ssid for current wifi network and find that in scan_networks list and return that network or else return an error with matching error code
        let scan_wifi_list = WifiModule::scan_wireless_network(&self).await?;
        let current_wifi = current_wifi
            .map(|x| {
                scan_wifi_list
                    .iter()
                    .find(|&y| y.name == x.ssid)
                    .map(|x| x.clone())
            })
            .flatten();

        match current_wifi {
            Some(current_wifi) => Ok(current_wifi.clone()),
            None => {
                trace_error!(task = "currnet_wifi", "unable to get current wifi network");
                bail!(WifiError::new(
                    WifiErrorCodes::UnableToGetWifiDeviceStatus,
                    format!("unable to get current wifi network"),
                ))
            }
        }
    }

    pub async fn connect_wireless_network(ssid: &str, psk: &str) -> Result<()> {
        trace!(
            task = "connect_wireless_network",
            "starting wifi connection"
        );

        let mut setup = match sta::WifiSetup::new() {
            Ok(setup) => {
                info!(task = "wifi_setup", "wifi setup successful");
                setup
            }
            Err(e) => {
                trace_error!(
                    task = "connect_wireless_network",
                    "unable to get wifi device status: {}",
                    e
                );
                bail!(WifiError::new(
                    WifiErrorCodes::UnableToGetWifiDeviceStatus,
                    format!("unable to get wifi device status: {}", e),
                ))
            }
        };

        let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
        setup.set_socket_path(proposed_path);

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let (_runtime, connect_wifi, _broadcast) = tokio::join!(
            async move {
                if let Err(e) = runtime.run().await {
                    trace_error!(task = "connect_wireless_network", "error: {}", e);
                }
            },
            WifiModule::connect_wifi(requester, &ssid, &psk),
            WifiModule::broadcast_listener(broadcast),
        );

        let wifi_list = match connect_wifi {
            Ok(wifi_list) => {
                info!(
                    task = "connect_wireless_network",
                    "wifi list: {:?}", wifi_list
                );
                wifi_list
            }
            Err(e) => {
                trace_error!(
                    task = "connect_wireless_network",
                    "unable to get wifi device status: {}",
                    e
                );
                bail!(WifiError::new(
                    WifiErrorCodes::UnableToConnectToWifiDevice,
                    format!("unable to connect to wifi network {}", e),
                ))
            }
        };

        Ok(wifi_list)
    }

    async fn connect_wifi(requester: sta::RequestClient, ssid: &str, psk: &str) -> Result<()> {
        trace!(task = "connect_wifi", "requesting networks");
        //handle networks or else return an error with matching error code
        let networks = match requester.get_networks().await {
            Ok(networks) => {
                info!(task = "connect_wifi", "networks: {:?}", networks);
                networks
            }
            Err(e) => {
                trace_error!(
                    task = "connect_wifi",
                    "unable to get wifi device status: {}",
                    e
                );
                bail!(WifiError::new(
                    WifiErrorCodes::UnableToConnectToWifiDevice,
                    format!("unable to connect to wifi network {}", e),
                ))
            }
        };

        //if ssid is in known networks, use that network id to connect else create new network id
        for network in networks {
            if network.ssid == ssid {
                info!("network id: {}", network.network_id);
                requester.select_network(network.network_id).await?;
                requester.shutdown().await?;
                return Ok(());
            }
        }

        //if ssid is not in known networks, create new network id and connect to it or else return an error with matching error code
        let network_id = match requester.add_network().await {
            Ok(network_id) => {
                info!(task = "connect_wifi", "network id: {}", network_id);
                network_id
            }
            Err(e) => {
                trace_error!(
                    task = "connect_wifi",
                    "unable to get wifi device status: {}",
                    e
                );
                bail!(WifiError::new(
                    WifiErrorCodes::UnableToConnectToWifiDevice,
                    format!("unable to connect to wifi network {}", e),
                ))
            }
        };

        //set network ssid
        requester
            .set_network_ssid(network_id, ssid.to_string())
            .await?;

        //set network psk
        requester
            .set_network_psk(network_id, psk.to_string())
            .await?;

        //select newly created network id or else return an error with matching error code
        let _ = match requester.select_network(network_id).await {
            Ok(_) => {
                info!(task = "connect_wifi", "connect to selected network");
                ()
            }
            Err(e) => {
                trace_error!(
                    task = "connect_wifi",
                    "unable to get wifi device status: {}",
                    e
                );
                bail!(WifiError::new(
                    WifiErrorCodes::UnableToConnectToWifiDevice,
                    format!("unable to connect to wifi network {}", e),
                ))
            }
        };

        requester.shutdown().await?;
        Ok(())
    }

    // remove wifi network from known networks using network id
    pub async fn remove_wireless_network(network_id: usize) -> Result<()> {
        trace!(task = "remove_wireless_network", "removing wifi network");

        let mut setup = match sta::WifiSetup::new() {
            Ok(setup) => {
                info!(task = "remove_wireless_network", "wifi setup successful");
                setup
            }
            Err(e) => {
                trace_error!(
                    task = "remove_wireless_network",
                    "unable to get wifi device status: {}",
                    e
                );
                bail!(WifiError::new(
                    WifiErrorCodes::UnableToGetWifiDeviceStatus,
                    format!("unable to get wifi device status: {}", e),
                ))
            }
        };

        let proposed_path = format!("/var/run/wpa_supplicant/wlan0");
        setup.set_socket_path(proposed_path);

        let broadcast = setup.get_broadcast_receiver();
        let requester = setup.get_request_client();
        let runtime = setup.complete();

        let (_runtime, remove_wifi, _broadcast) = tokio::join!(
            async move {
                if let Err(e) = runtime.run().await {
                    trace_error!(task = "remove_wireless_network", "error: {}", e);
                }
            },
            WifiModule::remove_wifi(requester, network_id),
            WifiModule::broadcast_listener(broadcast),
        );

        //use remove_wifi to remove the wifi network or else return an error with matching error code
        let wifi_list = match remove_wifi {
            Ok(wifi_list) => {
                info!(
                    task = "remove_wireless_network",
                    "wifi list: {:?}", wifi_list
                );
                wifi_list
            }
            Err(e) => {
                trace_error!(
                    task = "remove_wireless_network",
                    "unable to get wifi device status: {}",
                    e
                );
                bail!(WifiError::new(
                    WifiErrorCodes::UnableToRemoveWifiDevice,
                    format!("unable to remove wifi network {}", e),
                ))
            }
        };
        Ok(wifi_list)
    }

    async fn remove_wifi(requester: sta::RequestClient, network_id: usize) -> Result<()> {
        trace!(task = "remove_wifi", "removing wifi network");
        requester.remove_network(network_id).await?;
        requester.shutdown().await?;
        Ok(())
    }

    async fn broadcast_listener(mut broadcast_receiver: sta::BroadcastReceiver) -> Result<()> {
        trace!(task = "broadcast_listener", "listening for broadcasts");
        while let Ok(broadcast) = broadcast_receiver.recv().await {
            info!("Broadcast: {:?}", broadcast);
        }
        Ok(())
    }
}
