use std::io::{self, Read, Write};
use std::net::{SocketAddr, UdpSocket};

fn main() -> std::io::Result<()> {
    let addr = SocketAddr::from(([192, 168, 0, 104], 7568));
    let listener = UdpSocket::bind(addr)?;

    let mut buf = [0; 500];
    loop {
        let (number_of_bytes, _src_addr) = listener.recv_from(&mut buf)?;
        let mut filled_buf = &buf[0..number_of_bytes];
        //number_of_bytes 324
        //https://forums.forzamotorsport.net/turn10_postst128499_Forza-Motorsport-7--Data-Out--feature-details.aspx
        //https://forums.forzamotorsport.net/turn10_postsm1086008_Data-Output.aspx#post_1086008
        //0-231 sled data
        let mut byte4 = [0; 4];
        filled_buf.read(&mut byte4)?;
        let _is_race_on: i32 = i32::from_le_bytes(byte4);

        filled_buf.read(&mut byte4)?;
        let _time_stamp_ms: u32 = u32::from_le_bytes(byte4);

        filled_buf.read(&mut byte4)?;
        let _engine_max_rpm = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _engine_idle_rpm = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _engine_current_rpm = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _acceleration_x = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _acceleration_y = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _acceleration_z = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _velocity_x = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _velocity_y = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _velocity_z = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _angular_velocity_x = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _angular_velocity_y = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _angular_velocity_z = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _yaw = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _pitch = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _roll = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _normalized_suspension_travel_front_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _normalized_suspension_travel_front_right = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _normalized_suspension_travel_rear_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _normalized_suspension_travel_rear_right = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _tire_slip_ratio_front_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _tire_slip_ratio_front_right = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _tire_slip_ratio_rear_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _tire_slip_ratio_rear_right = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _wheel_rotation_speed_front_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _wheel_rotation_speed_front_right = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _wheel_rotation_speed_rear_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _wheel_rotation_speed_rear_right = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _wheel_on_rumble_strip_front_left = i32::from_le_bytes(byte4);
        filled_buf.read(&mut byte4)?;
        let _wheel_on_rumble_strip_front_right = i32::from_le_bytes(byte4);
        filled_buf.read(&mut byte4)?;
        let _wheel_on_rumble_strip_rear_left = i32::from_le_bytes(byte4);
        filled_buf.read(&mut byte4)?;
        let _wheel_on_rumble_strip_rear_right = i32::from_le_bytes(byte4);

        filled_buf.read(&mut byte4)?;
        let _wheel_in_puddle_depth_front_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _wheel_in_puddle_depth_front_right = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _wheel_in_puddle_depth_rear_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _wheel_in_puddle_depth_rear_right = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _surface_rumble_front_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _surface_rumble_front_right = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _surface_rumble_rear_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _surface_rumble_rear_right = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _tire_slip_angle_front_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _tire_slip_angle_front_right = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _tire_slip_angle_rear_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _tire_slip_angle_rear_right = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _tire_combined_slip_front_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _tire_combined_slip_front_right = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _tire_combined_slip_rear_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _tire_combined_slip_rear_right = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _suspension_travel_meters_front_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _suspension_travel_meters_front_right = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _suspension_travel_meters_rear_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _suspension_travel_meters_rear_right = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _car_ordinal = i32::from_le_bytes(byte4);
        filled_buf.read(&mut byte4)?;
        let _car_class = i32::from_le_bytes(byte4);
        filled_buf.read(&mut byte4)?;
        let _car_performance_index = i32::from_le_bytes(byte4);
        filled_buf.read(&mut byte4)?;
        let _drivetrain_type = i32::from_le_bytes(byte4);
        filled_buf.read(&mut byte4)?;
        let _num_cylinders = i32::from_le_bytes(byte4);

        //232-243 unknown data
        let mut byte12 = [0; 12];
        filled_buf.read(&mut byte12)?;

        //244-322 dash data
        filled_buf.read(&mut byte4)?;
        let _position_x = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _position_y = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _position_z = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _speed = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _power = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _torque = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _tire_temp_front_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _tire_temp_front_right = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _tire_temp_rear_left = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _tire_temp_rear_right = f32::from_bits(u32::from_le_bytes(byte4));

        filled_buf.read(&mut byte4)?;
        let _boost = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _fuel = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _distance_traveled = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _best_lap = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _last_lap = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _current_lap = f32::from_bits(u32::from_le_bytes(byte4));
        filled_buf.read(&mut byte4)?;
        let _current_race_time = f32::from_bits(u32::from_le_bytes(byte4));

        let mut byte2 = [0; 2];

        filled_buf.read(&mut byte2)?;
        let _lap_number = u16::from_le_bytes(byte2);

        let mut byte1 = [0; 1];

        filled_buf.read(&mut byte1)?;
        let _race_position = u8::from_le_bytes(byte1);
        filled_buf.read(&mut byte1)?;
        let _accel = u8::from_le_bytes(byte1);
        filled_buf.read(&mut byte1)?;
        let _brake = u8::from_le_bytes(byte1);
        filled_buf.read(&mut byte1)?;
        let _clutch = u8::from_le_bytes(byte1);
        filled_buf.read(&mut byte1)?;
        let _hand_brake = u8::from_le_bytes(byte1);
        filled_buf.read(&mut byte1)?;
        let _gear = u8::from_le_bytes(byte1);

        filled_buf.read(&mut byte1)?;
        let _steer = i8::from_le_bytes(byte1);
        filled_buf.read(&mut byte1)?;
        let _normalized_driving_line = i8::from_le_bytes(byte1);
        filled_buf.read(&mut byte1)?;
        let _normalized_ai_brake_difference = i8::from_le_bytes(byte1);

        //323 unknown data

        //when the string's length is longer than the cmd/powershell layout width, it still write in the new line
        //todo:how to refresh in different line
        print!("\rspeed:{}km/h  Torque:{}N.m  engineRPM:{}  carClass:{}  carPerInx:{}  drivertrainType:{}  numCylinders:{}", 
            (_speed*3.6).floor(),
            _torque.floor(),
            _engine_current_rpm.floor(),
            _car_class,
            _car_performance_index,
            _drivetrain_type,
            _num_cylinders
        );

        io::stdout().flush()?;
    }
}
