include!("ljx8_if.rs");
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_serial_number() -> Result<(), Box<dyn std::error::Error>> {
        let mut ljx8_if = LJX8If::ljx8_if_ethernet_open("192.168.0.2", 24691)?;
        let (controller_serial, head_serial) = ljx8_if.ljx8_if_get_serial_number()?;
        println!("controller_serial: {:?}", controller_serial);
        println!("head_serial: {:?}", head_serial);
        let (pn_sensor_temperature, pn_processor_temperature, pn_case_temperature) =
            ljx8_if.ljx8_if_get_head_temperature()?;
        println!("pnSensorTemperature: {:?}", pn_sensor_temperature);
        println!("pnProcessorTemperature: {:?}", pn_processor_temperature);
        println!("pnCaseTemperature: {:?}", pn_case_temperature);
        let result = ljx8_if.ljx8_if_communication_close()?;
        Ok(())
    }
    #[test]
    fn get_head_temperature() -> Result<(), Box<dyn std::error::Error>> {
        let mut ljx8_if = LJX8If::ljx8_if_ethernet_open("192.168.0.2", 24691)?;
        let (pn_sensor_temperature, pn_processor_temperature, pn_case_temperature) =
            ljx8_if.ljx8_if_get_head_temperature()?;
        println!("pnSensorTemperature: {:?}", pn_sensor_temperature);
        println!("pnProcessorTemperature: {:?}", pn_processor_temperature);
        println!("pnCaseTemperature: {:?}", pn_case_temperature);
        let result = ljx8_if.ljx8_if_communication_close()?;
        Ok(())
    }
}
