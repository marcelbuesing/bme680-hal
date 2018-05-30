use core::time::Duration;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum OversamplingSetting {
    OSNone = 0,
    OS1x = 1,
    OS2x = 2,
    OS4x = 3,
    OS8x = 4,
    OS16x = 5,
}

impl OversamplingSetting {
    // TODO replace with TryFrom once stabilized
    pub fn from_u8(os: u8) -> OversamplingSetting {
        match os {
            0 => OversamplingSetting::OSNone,
            1 => OversamplingSetting::OS1x,
            2 => OversamplingSetting::OS2x,
            3 => OversamplingSetting::OS4x,
            4 => OversamplingSetting::OS8x,
            5 => OversamplingSetting::OS16x,
            _ => panic!("Unknown oversampling setting: {}", os),
        }
    }
}

#[derive(Debug, Default, Copy)]
#[repr(C)]
pub struct TphSett {
    pub os_hum: Option<OversamplingSetting>,
    pub os_temp: Option<OversamplingSetting>,
    pub os_pres: Option<OversamplingSetting>,
    pub filter: Option<u8>,
}

impl Clone for TphSett {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug, Default, Copy)]
#[repr(C)]
pub struct GasSett {
    pub nb_conv: u8,
    /// Heater control
    pub heatr_ctrl: Option<u8>,
    /// Enable measurement of gas, disabled by default
    pub run_gas_measurement: bool,
    /// Heater temperature
    pub heatr_temp: Option<u16>,
    /// Profile duration
    pub heatr_dur: Option<Duration>,
    pub ambient_temperature: i8,
}

impl Clone for GasSett {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug, Default, Copy)]
pub struct SensorSettings {
    /// Gas settings
    pub gas_sett: GasSett,
    /// Temperature settings
    pub tph_sett: TphSett,
}

impl Clone for SensorSettings {
    fn clone(&self) -> Self {
        *self
    }
}

bitflags! {
    #[derive(Default)]
    pub struct DesiredSensorSettings: u16 {
        /// To set temperature oversampling
        const OST_SEL = 1;
        /// To set pressure oversampling.
        const OSP_SEL = 2;
        /// To set humidity oversampling.
        const OSH_SEL = 4;
        /// To set gas measurement setting.
        const GAS_MEAS_SEL = 8;
        /// To set filter setting.
        const FILTER_SEL = 16;
        /// To set humidity control setting.
        const HCNTRL_SEL = 32;
        /// To set run gas setting.
        const RUN_GAS_SEL = 64;
        /// To set NB conversion setting.
        const NBCONV_SEL = 128;
        /// To set all gas sensor related settings
        const GAS_SENSOR_SEL = Self::GAS_MEAS_SEL.bits | Self::RUN_GAS_SEL.bits | Self::NBCONV_SEL.bits;
    }
}

pub struct SettingsBuilder {
    desired_settings: DesiredSensorSettings,
    sensor_settings: SensorSettings,
}

pub type Settings = (SensorSettings, DesiredSensorSettings);

impl SettingsBuilder {
    pub fn new() -> SettingsBuilder {
        SettingsBuilder {
            desired_settings: Default::default(),
            sensor_settings: Default::default(),
        }
    }
    pub fn with_temperature_filter(mut self, filter: u8) -> SettingsBuilder {
        self.sensor_settings.tph_sett.filter = Some(filter);
        self.desired_settings |= DesiredSensorSettings::FILTER_SEL;
        self
    }

    pub fn with_humidity_control(mut self, heatr_control: u8) -> SettingsBuilder {
        self.sensor_settings.gas_sett.heatr_ctrl = Some(heatr_control);
        self.desired_settings |= DesiredSensorSettings::HCNTRL_SEL;
        self
    }

    pub fn with_temperature_oversampling(
        mut self,
        os_temp: OversamplingSetting,
    ) -> SettingsBuilder {
        self.sensor_settings.tph_sett.os_temp = Some(os_temp);
        self.desired_settings |= DesiredSensorSettings::OST_SEL;
        self
    }

    pub fn with_pressure_oversampling(mut self, os_pres: OversamplingSetting) -> SettingsBuilder {
        self.sensor_settings.tph_sett.os_pres = Some(os_pres);
        self.desired_settings |= DesiredSensorSettings::OSP_SEL;
        self
    }

    pub fn with_humidity_oversampling(mut self, os_hum: OversamplingSetting) -> SettingsBuilder {
        self.sensor_settings.tph_sett.os_hum = Some(os_hum);
        self.desired_settings |= DesiredSensorSettings::OSH_SEL;
        self
    }

    pub fn with_gas_measurement(
        mut self,
        heatr_dur: Duration,
        heatr_temp: u16,
        ambient_temperature: i8,
    ) -> SettingsBuilder {
        self.sensor_settings.gas_sett.heatr_dur = Some(heatr_dur);
        self.sensor_settings.gas_sett.heatr_temp = Some(heatr_temp);
        self.sensor_settings.gas_sett.ambient_temperature = ambient_temperature;
        self.desired_settings |= DesiredSensorSettings::RUN_GAS_SEL;
        self
    }

    pub fn with_nb_conv(mut self, nb_conv: u8) -> SettingsBuilder {
        self.sensor_settings.gas_sett.nb_conv = nb_conv;
        self.desired_settings |= DesiredSensorSettings::NBCONV_SEL;
        self
    }

    pub fn with_run_gas(mut self, run_gas: bool) -> SettingsBuilder {
        self.sensor_settings.gas_sett.run_gas_measurement = run_gas;
        self.desired_settings |= DesiredSensorSettings::RUN_GAS_SEL;
        self
    }

    pub fn build(self) -> Settings {
        (self.sensor_settings, self.desired_settings)
    }
}
