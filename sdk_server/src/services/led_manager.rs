use tonic::{Request, Response, Status};

pub use mecha_led_ctrl::{LedColor, LedCtrl};

#[allow(non_snake_case)]
pub mod ledmanager {
    tonic::include_proto!("led_ctrl");
}

pub use ledmanager::{
    led_ctrl_service_server::{LedCtrlService, LedCtrlServiceServer},
    Empty, LedColor as LedColorProto,
};

pub struct LedCtrlManager {
    pub led_ctrl: LedCtrl,
}

#[tonic::async_trait]
impl LedCtrlService for LedCtrlManager {
    async fn set_led(&self, request: Request<LedColorProto>) -> Result<Response<Empty>, Status> {
        let color = request.into_inner().color;
        // Match the color and convert it to the corresponding LedColor variant.
        let selected_led = match color {
            0 => LedColor::Red,
            1 => LedColor::Green,
            2 => LedColor::Blue,
            _ => {
                // Return an invalid argument status if the color is not recognized.
                return Err(Status::invalid_argument("Invalid color"));
            }
        };

        match self.led_ctrl.set_led(selected_led) {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(err) => Err(Status::from_error(err.into())),
        }
    }

    async fn clear_led(&self, request: Request<LedColorProto>) -> Result<Response<Empty>, Status> {
        let color = request.into_inner().color;

        // Match the color and convert it to the corresponding LedColor variant.
        let selected_led = match color {
            0 => LedColor::Red,
            1 => LedColor::Green,
            2 => LedColor::Blue,
            _ => {
                // Return an invalid argument status if the color is not recognized.
                return Err(Status::invalid_argument("Invalid color"));
            }
        };

        // Return an empty response if the LED was cleared successfully or else return error
        match self.led_ctrl.clear_led(selected_led) {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(err) => Err(Status::from_error(err.into())),
        }
    }
}
