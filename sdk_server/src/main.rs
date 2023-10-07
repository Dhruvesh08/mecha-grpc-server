use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::{fs::File, io::BufReader};
use tracing::{info, Level};
use tracing_subscriber;

use tonic::transport::Server;

mod configs;
use crate::configs::BaseConfig;

mod services;
use crate::services::{Battery, PowerSupply, PowerSupplyServiceServer};
use crate::services::{Bluetooth, BluetoothServiceServer};
use crate::services::{CpuCtrl, CpuCtrlService, CpuGovernorCtrlServiceServer};
use crate::services::{DeviceInfoCtrl, DeviceInfoServiceServer};
use crate::services::{DeviceMetrics, DeviceMetricsService, MetricsServiceServer};
use crate::services::{DisplayCtrl, DisplayCtrlManager, DisplayCtrlServiceServer};
use crate::services::{LedCtrl, LedCtrlManager, LedCtrlServiceServer};
use crate::services::{MotionSensor, MotionSensorManager, MotionSensorServiceServer};
use crate::services::{NetworkManager, NetworkManagerServiceServer};
use crate::services::{TrustZoneCtrl, TrustZoneCtrlServiceManager, TrustZoneCtrlServiceServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let profile_file = File::open("./Config.yaml").expect("Failed to open config file");
    let reader = BufReader::new(profile_file);

    let config: BaseConfig = serde_yaml::from_reader(reader).expect("unable to rad yaml file");

    //port for grpc server
    let port = config.server.port;
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port));

    //network manager service
    let network_service = NetworkManager::default();

    //display manager service
    let display_ctrl = DisplayCtrl::new(config.interfaces.display.device.as_str());
    let display_service = DisplayCtrlManager { display_ctrl };

    //motion sensor service
    let motion_sensor = MotionSensor::new(
        config.interfaces.motion_sensor.x_axis.as_str(),
        config.interfaces.motion_sensor.y_axis.as_str(),
        config.interfaces.motion_sensor.z_axis.as_str(),
    );
    let motion_sensor_manager = MotionSensorManager { motion_sensor };

    //led manager service
    let led_ctrl = LedCtrl::new(
        config.interfaces.led.red_led.as_str(),
        config.interfaces.led.green_led.as_str(),
        config.interfaces.led.blue_led.as_str(),
    );

    let led_manager = LedCtrlManager { led_ctrl };

    //device info service
    let device_info = DeviceInfoCtrl::default();

    //device metrics service
    let device_metrics = DeviceMetricsService {
        metrics: DeviceMetrics::new(),
    };

    //cpu governor service
    let cpu_governor = CpuCtrlService {
        cpu_ctrl_manager: CpuCtrl::new(),
    };

    //trustzone service
    let trustzone_ctrl = TrustZoneCtrlServiceManager {
        trustzone_ctrl: TrustZoneCtrl::new(),
    };

    let battery = Battery {
        path: config.interfaces.battery.device.as_str().to_string(),
        currnet_now: config.interfaces.battery.current.as_str().to_string(),
    };

    //power service
    let power_supply = PowerSupply {
        power_supply: battery,
    };

    println!("Mecha Edge Server listening on {}", addr);

    let subscriber = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        // build but do not install the subscriber.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!(
        task = "mecha_grpc_tracer",
        result = "success",
        "grpc server started"
    );
    Server::builder()
        .add_service(NetworkManagerServiceServer::new(network_service))
        .add_service(DisplayCtrlServiceServer::new(display_service))
        .add_service(MotionSensorServiceServer::new(motion_sensor_manager))
        .add_service(LedCtrlServiceServer::new(led_manager))
        .add_service(DeviceInfoServiceServer::new(device_info))
        .add_service(MetricsServiceServer::new(device_metrics))
        .add_service(CpuGovernorCtrlServiceServer::new(cpu_governor))
        .add_service(TrustZoneCtrlServiceServer::new(trustzone_ctrl))
        .add_service(PowerSupplyServiceServer::new(power_supply))
        .add_service(BluetoothServiceServer::new(Bluetooth::default()))
        .serve(addr)
        .await?;
    Ok(())
}
