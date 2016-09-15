/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_adapter::BluetoothAdapter as BluetoothAdapterBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_adapter::Adapter as BluetoothAdapterAndroid;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
use empty::BluetoothAdapter as BluetoothAdapterEmpty;
#[cfg(feature = "bluetooth")]
use blurmock::fake_adapter::FakeBluetoothAdapter;
#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_device::BluetoothDevice as BluetoothDeviceBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_device::Device as BluetoothDeviceAndroid;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
use empty::BluetoothDevice as BluetoothDeviceEmpty;
#[cfg(feature = "bluetooth")]
use blurmock::fake_device::FakeBluetoothDevice;
#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic as BluetoothGATTCharacteristicBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_gatt_characteristic::Characteristic as BluetoothGATTCharacteristicAndroid;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
use empty::BluetoothGATTCharacteristic as BluetoothGATTCharacteristicEmpty;
#[cfg(feature = "bluetooth")]
use blurmock::fake_characteristic::FakeBluetoothGATTCharacteristic;
#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_gatt_descriptor::BluetoothGATTDescriptor as BluetoothGATTDescriptorBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_gatt_descriptor::Descriptor as BluetoothGATTDescriptorAndroid;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
use empty::BluetoothGATTDescriptor as BluetoothGATTDescriptorEmpty;
#[cfg(feature = "bluetooth")]
use blurmock::fake_descriptor::FakeBluetoothGATTDescriptor;
#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_gatt_service::BluetoothGATTService as BluetoothGATTServiceBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_gatt_service::Service as BluetoothGATTServiceAndroid;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
use empty::BluetoothGATTService as BluetoothGATTServiceEmpty;
#[cfg(feature = "bluetooth")]
use blurmock::fake_service::FakeBluetoothGATTService;
#[cfg(all(target_os = "linux", feature = "bluetooth"))]
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as BluetoothDiscoverySessionBluez;
#[cfg(all(target_os = "android", feature = "bluetooth"))]
use blurdroid::bluetooth_discovery_session::DiscoverySession as BluetoothDiscoverySessionAndroid;
#[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
use empty::BluetoothDiscoverySession as BluetoothDiscoverySessionEmpty;
#[cfg(feature = "bluetooth")]
use blurmock::fake_discovery_session::FakeBluetoothDiscoverySession;

use std::sync::Arc;
use std::error::Error;

const NOT_SUPPORTED_ERROR: &'static str = "Error! Not supported function!";

#[derive(Clone, Debug)]
pub enum BluetoothAdapter {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    Bluez(Arc<BluetoothAdapterBluez>),
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    Android(Arc<BluetoothAdapterAndroid>),
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
    Empty(Arc<BluetoothAdapterEmpty>),
    #[cfg(feature = "bluetooth")]
    Mock(Arc<FakeBluetoothAdapter>),
}

#[derive(Debug)]
pub enum BluetoothDiscoverySession {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    Bluez(Arc<BluetoothDiscoverySessionBluez>),
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    Android(Arc<BluetoothDiscoverySessionAndroid>),
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
    Empty(Arc<BluetoothDiscoverySessionEmpty>),
    #[cfg(feature = "bluetooth")]
    Mock(Arc<FakeBluetoothDiscoverySession>),
}

#[derive(Clone, Debug)]
pub enum BluetoothDevice {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    Bluez(Arc<BluetoothDeviceBluez>),
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    Android(Arc<BluetoothDeviceAndroid>),
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
    Empty(Arc<BluetoothDeviceEmpty>),
    #[cfg(feature = "bluetooth")]
    Mock(Arc<FakeBluetoothDevice>),
}

#[derive(Clone, Debug)]
pub enum BluetoothGATTService {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    Bluez(Arc<BluetoothGATTServiceBluez>),
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    Android(Arc<BluetoothGATTServiceAndroid>),
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
    Empty(Arc<BluetoothGATTServiceEmpty>),
    #[cfg(feature = "bluetooth")]
    Mock(Arc<FakeBluetoothGATTService>),
}

#[derive(Clone, Debug)]
pub enum BluetoothGATTCharacteristic {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    Bluez(Arc<BluetoothGATTCharacteristicBluez>),
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    Android(Arc<BluetoothGATTCharacteristicAndroid>),
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
    Empty(Arc<BluetoothGATTCharacteristicEmpty>),
    #[cfg(feature = "bluetooth")]
    Mock(Arc<FakeBluetoothGATTCharacteristic>),
}

#[derive(Clone, Debug)]
pub enum BluetoothGATTDescriptor {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    Bluez(Arc<BluetoothGATTDescriptorBluez>),
    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    Android(Arc<BluetoothGATTDescriptorAndroid>),
    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
    Empty(Arc<BluetoothGATTDescriptorEmpty>),
    #[cfg(feature = "bluetooth")]
    Mock(Arc<FakeBluetoothGATTDescriptor>),
}

impl BluetoothAdapter {
    #[cfg(all(target_os = "linux", feature = "bluetooth"))]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapterBluez::init());
        Ok(BluetoothAdapter::Bluez(Arc::new(bluez_adapter)))
    }

    #[cfg(all(target_os = "android", feature = "bluetooth"))]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        let blurdroid_adapter = try!(BluetoothAdapterAndroid::get_adapter());
        Ok(BluetoothAdapter::Android(Arc::new(blurdroid_adapter)))
    }

    #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        let adapter = try!(BluetoothAdapterEmpty::init());
        Ok(BluetoothAdapter::Empty(Arc::new(adapter)))
    }

    #[cfg(feature = "bluetooth")]
    pub fn init_mock() -> Result<BluetoothAdapter, Box<Error>> {
        Ok(BluetoothAdapter::Mock(FakeBluetoothAdapter::new_empty()))
    }

    pub fn get_id(&self) -> String {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_id(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_id(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_id(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_id(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_id(&self, id: String) {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_id(id),
            _ => (),
        }
    }

    pub fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>> {
        let device_list = match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => try!(bluez_adapter.get_device_list()),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => try!(android_adapter.get_device_list()),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => try!(adapter.get_device_list()),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => try!(fake_adapter.get_device_list()),
        };
        Ok(device_list.into_iter().map(|device| BluetoothDevice::create_device(self.clone(), device)).collect())
    }

    /*pub fn get_device(&self, address: String) -> Result<Option<BluetoothDevice>, Box<Error>> {
        let devices = try!(self.get_devices());
        for device in devices {
            if try!(device.get_address()) == address {
                return Ok(Some(device));
            }
        }
        Ok(None)
    }*/

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_address(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_address(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_address(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_address(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_address(&self, address: String) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_address(address),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_name(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_name(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_name(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_name(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_name(&self, name: String) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_name(name),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_alias(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_alias(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_alias(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_alias(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_alias(&self, value: String) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_alias(value),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_class(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_class(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_class(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_class(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_class(&self, class: u32) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_class(class),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.is_powered(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.is_powered(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.is_powered(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.is_powered(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_powered(&self, value: bool) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_powered(value),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn is_present(&self) -> Result<bool, Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.is_present(),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_present(&self, value: bool) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_present(value),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.is_discoverable(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.is_discoverable(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.is_discoverable(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.is_discoverable(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_discoverable(&self, value: bool) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_discoverable(value),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn is_pairable(&self) -> Result<bool, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.is_pairable(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.is_pairable(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.is_pairable(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.is_pairable(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_pairable(&self, value: bool) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_pairable(value),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_pairable_timeout(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_pairable_timeout(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_pairable_timeout(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_pairable_timeout(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_pairable_timeout(&self, value: u32) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_pairable_timeout(value),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_discoverable_timeout(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_discoverable_timeout(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_discoverable_timeout(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_discoverable_timeout(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_discoverable_timeout(&self, value: u32) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_discoverable_timeout(value),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn is_discovering(&self) -> Result<bool, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.is_discovering(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.is_discovering(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.is_discovering(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.is_discovering(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_discovering(&self, is_discovering: bool) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_discovering(is_discovering),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_can_start_discovery(&self, can_start_discovery: bool) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_can_start_discovery(can_start_discovery),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn create_discovery_session(&self) -> Result<BluetoothDiscoverySession, Box<Error>> {
        BluetoothDiscoverySession::create_session(self.clone())
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_uuids(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_uuids(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_uuids(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_uuids(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_uuids(uuids),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_vendor_id_source(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_vendor_id_source(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_vendor_id_source(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_vendor_id_source(),
        }
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_vendor_id(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_vendor_id(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_vendor_id(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_vendor_id(),
        }
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_product_id(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_product_id(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_product_id(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_product_id(),
        }
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_device_id(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_device_id(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_device_id(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_device_id(),
        }
    }

    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothAdapter::Bluez(ref bluez_adapter) => bluez_adapter.get_modalias(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothAdapter::Android(ref android_adapter) => android_adapter.get_modalias(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothAdapter::Empty(ref adapter) => adapter.get_modalias(),
            #[cfg(feature = "bluetooth")]
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.get_modalias(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_modalias(&self, modalias: String) -> Result<(), Box<Error>> {
        match self {
            &BluetoothAdapter::Mock(ref fake_adapter) => fake_adapter.set_modalias(modalias),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }
}

impl BluetoothDiscoverySession {
    pub fn create_session(adapter: BluetoothAdapter) -> Result<BluetoothDiscoverySession, Box<Error>> {
        match adapter {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            BluetoothAdapter::Bluez(bluez_adapter) => {
                let bluez_session = try!(BluetoothDiscoverySessionBluez::create_session(bluez_adapter.get_id()));
                Ok(BluetoothDiscoverySession::Bluez(Arc::new(bluez_session)))
            },
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            BluetoothAdapter::Android(android_adapter) => {
                let blurdroid_session = try!(BluetoothDiscoverySessionAndroid::create_session(android_adapter));
                Ok(BluetoothDiscoverySession::Android(Arc::new(blurdroid_session)))
            },
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            BluetoothAdapter::Empty(adapter) => {
                let empty_session = try!(BluetoothDiscoverySessionEmpty::create_session(adapter.get_adapter()));
                Ok(BluetoothDiscoverySession::Empty(Arc::new(empty_session)))
            },
            #[cfg(feature = "bluetooth")]
            BluetoothAdapter::Mock(fake_adapter) => {
                let test_session = try!(FakeBluetoothDiscoverySession::create_session(fake_adapter));
                Ok(BluetoothDiscoverySession::Mock(Arc::new(test_session)))
            },
        }
    }

    pub fn start_discovery(&self) -> Result<(), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDiscoverySession::Bluez(ref bluez_session) => bluez_session.start_discovery(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDiscoverySession::Android(ref android_session) => android_session.start_discovery(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDiscoverySession::Empty(ref adapter) => adapter.start_discovery(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDiscoverySession::Mock(ref fake_session) => fake_session.start_discovery(),
        }
    }

    pub fn stop_discovery(&self) -> Result<(), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDiscoverySession::Bluez(ref bluez_session) => bluez_session.stop_discovery(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDiscoverySession::Android(ref android_session) => android_session.stop_discovery(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDiscoverySession::Empty(ref adapter) => adapter.stop_discovery(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDiscoverySession::Mock(ref fake_session) => fake_session.stop_discovery(),
        }
    }
}

impl BluetoothDevice {

    pub fn create_device(adapter: BluetoothAdapter, device: String) -> BluetoothDevice {
        match adapter {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            BluetoothAdapter::Bluez(_bluez_adapter) => {
                BluetoothDevice::Bluez(Arc::new(BluetoothDeviceBluez::new(device)))
            },
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            BluetoothAdapter::Android(android_adapter) => {
                BluetoothDevice::Android(Arc::new(BluetoothDeviceAndroid::new(android_adapter, device)))
            },
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            BluetoothAdapter::Empty(adapter) => {
                BluetoothDevice::Empty(Arc::new(BluetoothDeviceEmpty::new(device)))
            },
            #[cfg(feature = "bluetooth")]
            BluetoothAdapter::Mock(fake_adapter) => {
                BluetoothDevice::Mock(FakeBluetoothDevice::new_empty(fake_adapter, device))
            },
        }
    }

    pub fn get_id(&self) -> String {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_id(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_id(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_id(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_id(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_id(&self, id: String) {
        match self {
            &BluetoothDevice::Mock(ref fake_device) => fake_device.set_id(id),
            _ => (),
        }
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_address(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_address(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_address(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_address(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_address(&self, address: String) -> Result<(), Box<Error>> {
        match self {
            &BluetoothDevice::Mock(ref fake_device) => fake_device.set_address(address),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_name(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_name(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_name(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_name(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_name(&self, name: String) -> Result<(), Box<Error>> {
        match self {
            &BluetoothDevice::Mock(ref fake_device) => fake_device.set_name(name),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn get_icon(&self) -> Result<String, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_icon(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_icon(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_icon(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_icon(),
        }
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_class(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_class(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_class(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_class(),
        }
    }

    pub fn get_appearance(&self) -> Result<u16, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_appearance(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_appearance(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_appearance(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_appearance(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_appearance(&self, appearance: u16) -> Result<(), Box<Error>> {
        match self {
            &BluetoothDevice::Mock(ref fake_device) => fake_device.set_appearance(appearance),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_uuids(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_uuids(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_uuids(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_uuids(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>> {
        match self {
            &BluetoothDevice::Mock(ref fake_device) => fake_device.set_uuids(uuids),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn is_paired(&self) -> Result<bool, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.is_paired(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.is_paired(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.is_paired(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.is_paired(),
        }
    }

    pub fn is_connected(&self) -> Result<bool, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.is_connected(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.is_connected(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.is_connected(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.is_connected(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn is_connectable(&self) -> Result<bool, Box<Error>> {
        match self {
            &BluetoothDevice::Mock(ref fake_device) => fake_device.is_connectable(),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_connectable(&self, connectable: bool) -> Result<(), Box<Error>> {
        match self {
            &BluetoothDevice::Mock(ref fake_device) => fake_device.set_connectable(connectable),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn is_trusted(&self) -> Result<bool, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.is_trusted(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.is_trusted(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.is_trusted(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.is_trusted(),
        }
    }

    pub fn is_blocked(&self) -> Result<bool, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.is_blocked(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.is_blocked(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.is_blocked(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.is_blocked(),
        }
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_alias(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_alias(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_alias(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_alias(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_alias(&self, alias: String) -> Result<(), Box<Error>> {
        match self {
            &BluetoothDevice::Mock(ref fake_device) => fake_device.set_alias(alias),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn is_legacy_pairing(&self) -> Result<bool, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.is_legacy_pairing(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.is_legacy_pairing(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.is_legacy_pairing(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.is_legacy_pairing(),
        }
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_vendor_id_source(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_vendor_id_source(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_vendor_id_source(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_vendor_id_source(),
        }
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_vendor_id(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_vendor_id(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_vendor_id(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_vendor_id(),
        }
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_product_id(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_product_id(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_product_id(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_product_id(),
        }
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_device_id(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_device_id(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_device_id(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_device_id(),
        }
    }

    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_modalias(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_modalias(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_modalias(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_modalias(),
        }
    }

    pub fn get_rssi(&self) -> Result<i16, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_rssi(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_rssi(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_rssi(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_rssi(),
        }
    }

    pub fn get_tx_power(&self) -> Result<i16, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.get_tx_power(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.get_tx_power(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.get_tx_power(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.get_tx_power(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_tx_power(&self, tx_power: i16) -> Result<(), Box<Error>> {
        match self {
            &BluetoothDevice::Mock(ref fake_device) => fake_device.set_tx_power(tx_power),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn get_gatt_services(&self) -> Result<Vec<BluetoothGATTService>, Box<Error>> {
        let services = match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => try!(bluez_device.get_gatt_services()),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => try!(android_device.get_gatt_services()),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref adapter) => try!(adapter.get_gatt_services()),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => try!(fake_device.get_gatt_services()),
        };
        Ok(services.into_iter().map(|service| BluetoothGATTService::create_service(self.clone(), service)).collect())
    }

    pub fn connect(&self) -> Result<(), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.connect(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.connect(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.connect(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.connect(),
        }
    }

    pub fn disconnect(&self) -> Result<(), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.disconnect(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.disconnect(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.disconnect(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.disconnect(),
        }
    }

    pub fn connect_profile(&self, uuid: String) -> Result<(), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.connect_profile(uuid),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.connect_profile(uuid),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.connect_profile(uuid),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.connect_profile(uuid),
        }
    }

    pub fn disconnect_profile(&self, uuid: String) -> Result<(), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.disconnect_profile(uuid),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.disconnect_profile(uuid),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.disconnect_profile(uuid),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.disconnect_profile(uuid),
        }
    }

    pub fn pair(&self) -> Result<(), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.pair(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.pair(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.pair(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.pair(),
        }
    }

    pub fn cancel_pairing(&self) -> Result<(), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothDevice::Bluez(ref bluez_device) => bluez_device.cancel_pairing(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothDevice::Android(ref android_device) => android_device.cancel_pairing(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothDevice::Empty(ref device) => device.cancel_pairing(),
            #[cfg(feature = "bluetooth")]
            &BluetoothDevice::Mock(ref fake_device) => fake_device.cancel_pairing(),
        }
    }
}

impl BluetoothGATTService {
    pub fn create_service(device: BluetoothDevice, service: String) -> BluetoothGATTService {
        match device {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            BluetoothDevice::Bluez(_bluez_device) => {
                BluetoothGATTService::Bluez(Arc::new(BluetoothGATTServiceBluez::new(service)))
            },
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            BluetoothDevice::Android(android_device) => {
                BluetoothGATTService::Android(Arc::new(BluetoothGATTServiceAndroid::new(android_device, service)))
            },
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            BluetoothDevice::Empty(device) => {
                BluetoothGATTService::Empty(Arc::new(BluetoothGATTServiceEmpty::new(service)))
            },
            #[cfg(feature = "bluetooth")]
            BluetoothDevice::Mock(fake_device) => {
                BluetoothGATTService::Mock(FakeBluetoothGATTService::new_empty(fake_device, service))
            },
        }
    }

    pub fn get_id(&self) -> String {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTService::Bluez(ref bluez_service) => bluez_service.get_id(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTService::Android(ref android_service) => android_service.get_id(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTService::Empty(ref service) => service.get_id(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTService::Mock(ref fake_service) => fake_service.get_id(),
        }
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTService::Bluez(ref bluez_service) => bluez_service.get_uuid(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTService::Android(ref android_service) => android_service.get_uuid(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTService::Empty(ref service) => service.get_uuid(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTService::Mock(ref fake_service) => fake_service.get_uuid(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_uuid(&self, uuid: String) -> Result<(), Box<Error>> {
        match self {
            &BluetoothGATTService::Mock(ref fake_service) => fake_service.set_uuid(uuid),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn is_primary(&self) -> Result<bool, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTService::Bluez(ref bluez_service) => bluez_service.is_primary(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTService::Android(ref android_service) => android_service.is_primary(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTService::Empty(ref service) => service.is_primary(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTService::Mock(ref fake_service) => fake_service.is_primary(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_primary(&self, value: bool) -> Result<(), Box<Error>> {
        match self {
            &BluetoothGATTService::Mock(ref fake_service) => fake_service.set_is_primary(value),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn get_includes(&self) -> Result<Vec<BluetoothGATTService>, Box<Error>> {
        /*let services = try!(self.get_gatt_service().get_includes());
        Ok(services.into_iter().map(|service| BluetoothGATTService::create_service(self.get_device(), service)).collect())*/
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    pub fn get_gatt_characteristics(&self) -> Result<Vec<BluetoothGATTCharacteristic>, Box<Error>> {
        let characteristics = match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTService::Bluez(ref bluez_service) => try!(bluez_service.get_gatt_characteristics()),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTService::Android(ref android_service) => try!(android_service.get_gatt_characteristics()),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTService::Empty(ref service) => try!(service.get_gatt_characteristics()),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTService::Mock(ref fake_service) => try!(fake_service.get_gatt_characteristics()),
        };
        Ok(characteristics.into_iter().map(|characteristic| BluetoothGATTCharacteristic::create_characteristic(self.clone(), characteristic)).collect())
    }
}

impl BluetoothGATTCharacteristic {
    pub fn create_characteristic(service: BluetoothGATTService, characteristic: String) -> BluetoothGATTCharacteristic {
        match service {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            BluetoothGATTService::Bluez(_bluez_service) => {
                BluetoothGATTCharacteristic::Bluez(Arc::new(BluetoothGATTCharacteristicBluez::new(characteristic)))
            },
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            BluetoothGATTService::Android(android_service) => {
                BluetoothGATTCharacteristic::Android(Arc::new(BluetoothGATTCharacteristicAndroid::new(android_service, characteristic)))
            },
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            BluetoothGATTService::Empty(service) => {
                BluetoothGATTCharacteristic::Empty(Arc::new(BluetoothGATTCharacteristicEmpty::new(characteristic)))
            },
            #[cfg(feature = "bluetooth")]
            BluetoothGATTService::Mock(fake_service) => {
                BluetoothGATTCharacteristic::Mock(FakeBluetoothGATTCharacteristic::new_empty(fake_service, characteristic))
            },
        }
    }

    pub fn get_id(&self) -> String {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Bluez(ref bluez_characteristic) => bluez_characteristic.get_id(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Android(ref android_characteristic) => android_characteristic.get_id(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTCharacteristic::Empty(ref characteristic) => characteristic.get_id(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTCharacteristic::Mock(ref fake_characteristic) => fake_characteristic.get_id(),
        }
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Bluez(ref bluez_characteristic) => bluez_characteristic.get_uuid(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Android(ref android_characteristic) => android_characteristic.get_uuid(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTCharacteristic::Empty(ref characteristic) => characteristic.get_uuid(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTCharacteristic::Mock(ref fake_characteristic) => fake_characteristic.get_uuid(),
        }
    }

    #[cfg(feature = "bluetooth")]
    pub fn set_uuid(&self, uuid: String) -> Result<(), Box<Error>> {
        match self {
            &BluetoothGATTCharacteristic::Mock(ref fake_characteristic) => fake_characteristic.set_uuid(uuid),
            _ => Err(Box::from(NOT_SUPPORTED_ERROR)),
        }
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Bluez(ref bluez_characteristic) => bluez_characteristic.get_value(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Android(ref android_characteristic) => android_characteristic.get_value(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTCharacteristic::Empty(ref characteristic) => characteristic.get_value(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTCharacteristic::Mock(ref fake_characteristic) => fake_characteristic.get_value(),
        }
    }

    pub fn is_notifying(&self) -> Result<bool, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Bluez(ref bluez_characteristic) => bluez_characteristic.is_notifying(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Android(ref android_characteristic) => android_characteristic.is_notifying(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTCharacteristic::Empty(ref characteristic) => characteristic.is_notifying(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTCharacteristic::Mock(ref fake_characteristic) => fake_characteristic.is_notifying(),
        }
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Bluez(ref bluez_characteristic) => bluez_characteristic.get_flags(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Android(ref android_characteristic) => android_characteristic.get_flags(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTCharacteristic::Empty(ref characteristic) => characteristic.get_flags(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTCharacteristic::Mock(ref fake_characteristic) => fake_characteristic.get_flags(),
        }
    }

    pub fn get_gatt_descriptors(&self) -> Result<Vec<BluetoothGATTDescriptor>, Box<Error>> {
        let descriptors =  match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Bluez(ref bluez_characteristic) => try!(bluez_characteristic.get_gatt_descriptors()),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Android(ref android_characteristic) => try!(android_characteristic.get_gatt_descriptors()),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTCharacteristic::Empty(ref characteristic) => try!(characteristic.get_gatt_descriptors()),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTCharacteristic::Mock(ref fake_characteristic) => try!(fake_characteristic.get_gatt_descriptors()),
        };
        Ok(descriptors.into_iter().map(|descriptor| BluetoothGATTDescriptor::create_descriptor(self.clone(), descriptor)).collect())
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Bluez(ref bluez_characteristic) => bluez_characteristic.read_value(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Android(ref android_characteristic) => android_characteristic.read_value(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTCharacteristic::Empty(ref characteristic) => characteristic.read_value(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTCharacteristic::Mock(ref fake_characteristic) => fake_characteristic.read_value(),
        }
    }

    pub fn write_value(&self, values: Vec<u8>) -> Result<(), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Bluez(ref bluez_characteristic) => bluez_characteristic.write_value(values),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Android(ref android_characteristic) => android_characteristic.write_value(values),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTCharacteristic::Empty(ref characteristic) => characteristic.write_value(values),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTCharacteristic::Mock(ref fake_characteristic) => fake_characteristic.write_value(values),
        }
    }

    pub fn start_notify(&self) -> Result<(), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Bluez(ref bluez_characteristic) => bluez_characteristic.start_notify(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Android(ref android_characteristic) => android_characteristic.start_notify(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTCharacteristic::Empty(ref characteristic) => characteristic.start_notify(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTCharacteristic::Mock(ref fake_characteristic) => fake_characteristic.start_notify(),
        }
    }

    pub fn stop_notify(&self) -> Result<(), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Bluez(ref bluez_characteristic) => bluez_characteristic.stop_notify(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTCharacteristic::Android(ref android_characteristic) => android_characteristic.stop_notify(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTCharacteristic::Empty(ref characteristic) => characteristic.stop_notify(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTCharacteristic::Mock(ref fake_characteristic) => fake_characteristic.stop_notify(),
        }
    }
}

impl BluetoothGATTDescriptor {
    pub fn create_descriptor(characteristic: BluetoothGATTCharacteristic, descriptor: String) -> BluetoothGATTDescriptor {
        match characteristic {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            BluetoothGATTCharacteristic::Bluez(_bluez_characteristic) => {
                BluetoothGATTDescriptor::Bluez(Arc::new(BluetoothGATTDescriptorBluez::new(descriptor)))
            },
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            BluetoothGATTCharacteristic::Android(android_characteristic) => {
                BluetoothGATTDescriptor::Android(Arc::new(BluetoothGATTDescriptorAndroid::new(android_characteristic, descriptor)))
            },
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            BluetoothGATTCharacteristic::Empty(characteristic) => {
                BluetoothGATTDescriptor::Empty(Arc::new(BluetoothGATTDescriptorEmpty::new(descriptor)))
            },
            #[cfg(feature = "bluetooth")]
            BluetoothGATTCharacteristic::Mock(fake_characteristic) => {
                BluetoothGATTDescriptor::Mock(FakeBluetoothGATTDescriptor::new_empty(fake_characteristic, descriptor))
            },
        }
    }

    pub fn get_id(&self) -> String {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTDescriptor::Bluez(ref bluez_descriptor) => bluez_descriptor.get_id(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTDescriptor::Android(ref android_descriptor) => android_descriptor.get_id(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTDescriptor::Empty(ref descriptor) => descriptor.get_id(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTDescriptor::Mock(ref fake_descriptor) => fake_descriptor.get_id(),
        }
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTDescriptor::Bluez(ref bluez_descriptor) => bluez_descriptor.get_uuid(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTDescriptor::Android(ref android_descriptor) => android_descriptor.get_uuid(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTDescriptor::Empty(ref descriptor) => descriptor.get_uuid(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTDescriptor::Mock(ref fake_descriptor) => fake_descriptor.get_uuid(),
        }
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTDescriptor::Bluez(ref bluez_descriptor) => bluez_descriptor.get_value(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTDescriptor::Android(ref android_descriptor) => android_descriptor.get_value(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTDescriptor::Empty(ref descriptor) => descriptor.get_value(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTDescriptor::Mock(ref fake_descriptor) => fake_descriptor.get_value(),
        }
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTDescriptor::Bluez(ref bluez_descriptor) => bluez_descriptor.get_flags(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTDescriptor::Android(ref android_descriptor) => android_descriptor.get_flags(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTDescriptor::Empty(ref descriptor) => descriptor.get_flags(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTDescriptor::Mock(ref fake_descriptor) => fake_descriptor.get_flags(),
        }
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTDescriptor::Bluez(ref bluez_descriptor) => bluez_descriptor.read_value(),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTDescriptor::Android(ref android_descriptor) => android_descriptor.read_value(),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTDescriptor::Empty(ref descriptor) => descriptor.read_value(),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTDescriptor::Mock(ref fake_descriptor) => fake_descriptor.read_value(),
        }
    }

    pub fn write_value(&self, values: Vec<u8>) -> Result<(), Box<Error>> {
        match self {
            #[cfg(all(target_os = "linux", feature = "bluetooth"))]
            &BluetoothGATTDescriptor::Bluez(ref bluez_descriptor) => bluez_descriptor.write_value(values),
            #[cfg(all(target_os = "android", feature = "bluetooth"))]
            &BluetoothGATTDescriptor::Android(ref android_descriptor) => android_descriptor.write_value(values),
            #[cfg(not(any(all(target_os = "linux", feature = "bluetooth"), all(target_os = "android", feature = "bluetooth"))))]
            &BluetoothGATTDescriptor::Empty(ref descriptor) => descriptor.write_value(values),
            #[cfg(feature = "bluetooth")]
            &BluetoothGATTDescriptor::Mock(ref fake_descriptor) => fake_descriptor.write_value(values),
        }
    }
}
