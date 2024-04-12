use std::{borrow::Cow, io::Write};

use valence_protocol::{
    packets::play::entity_equipment_update_s2c::EquipmentEntry, Decode, Encode, ItemStack, Packet,
    VarInt,
};

#[derive(Clone, PartialEq, Debug, Packet)]
pub struct EntityEquipmentUpdateS2c<'a> {
    pub entity_id: VarInt,
    pub equipment: Cow<'a, [EquipmentEntry]>,
}

impl Encode for EntityEquipmentUpdateS2c<'_> {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        self.entity_id.encode(&mut w)?;

        for i in 0..self.equipment.len() {
            let slot = self.equipment[i].slot;
            if i == self.equipment.len() - 1 {
                slot.encode(&mut w)?;
            } else {
                (slot | -128).encode(&mut w)?;
            }
            self.equipment[i].item.encode(&mut w)?;
        }

        Ok(())
    }
}

impl<'a> Decode<'a> for EntityEquipmentUpdateS2c<'static> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let entity_id = VarInt::decode(r)?;

        let mut equipment = vec![];

        loop {
            let slot = i8::decode(r)?;
            let item = ItemStack::decode(r)?;
            equipment.push(EquipmentEntry {
                slot: slot & 127,
                item,
            });
            if slot & -128 == 0 {
                break;
            }
        }

        Ok(Self {
            entity_id,
            equipment: Cow::Owned(equipment),
        })
    }
}
