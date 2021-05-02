use bevy::prelude::*;

/// The address that has been typed into the address bar.
pub struct Address(pub String);

pub struct AddressEntered {
    pub address: String, // TODO: The address should be an Address
}

fn address_entered(mut event_reader: EventReader<AddressEntered>) {
    for event in event_reader.iter() {
        println!("Got AddressEntered event with address: {}", event.address);
        // TODO: Start HTTP request
    }
}

pub struct AddressPlugin;

impl Plugin for AddressPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AddressEntered>()
            .add_system(address_entered.system());
    }
}
