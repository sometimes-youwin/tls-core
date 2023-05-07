use serde::{Deserialize, Serialize};

macro_rules! stats {
    ($( $( #[$attr:meta] )* $name:ident : $type:ty { $( $( #[$stat_attr:meta] )* $stat:ident ),+ } )+) => {
        $(
            $(#[$attr])*
            #[derive(Debug, Default, Hash, Serialize, Deserialize)]
            pub struct $name {
                $(
                    $(#[$stat_attr])*
                    #[serde(default)]
                    $stat: $type,
                )+
            }
        )+
    };
}

stats! {
    /// The raw attributes for each unit.
    Attributes: u64 {
        /// Affects melee damage, carrying capacity, and health.
        strength,
        /// Affects defense, resistances, carrying capacity, and health.
        endurance,
        /// Affects ranged damage, hit rate, and critical hit rate.
        dexterity,
        /// Affects speed, carrying capacity, and dodge rate.
        agility,
        /// Affects tech damage, magic damage, and equipment.
        intelligence,
        /// Affects resistances, hit rate, and dodge rate.
        wisdom
    }

    Equipment: u64 {
        // ---
        // Melee weapons
        // ---

        /// Hands, feet, head, shoulders, etc.
        unarmed,
        /// Brass knuckles, rings, etc.
        knuckles,
        /// Bricks, crates, rubber ducks, etc.
        improvised_weapons,

        daggers,
        one_handed_swords,
        two_handed_swords,

        one_handed_maces,
        two_handed_maces,

        one_handed_axes,
        two_handed_axes,

        staves,
        spears,
        polearms,

        /// Bucklers, heater shields, etc.
        light_shields,
        /// Pavise shields, targes, kite shields, etc.
        heavy_shields,

        // ---
        // Ranged weapons
        // ---

        thrown_projectiles,
        /// Mortars, fireworks, etc.
        emplaced_projectiles,
        slingshots,

        /// Recurve bows, compound bows, etc.
        bows,
        /// Crossbows of all types.
        crossbows,

        /// Magazine/clip-fed handguns.
        pistols,
        /// Revolving chamber handguns.
        revolvers,
        /// Pistol caliber, automatic weapons larger than pistols.
        submachine_guns,

        /// Firearms that can take many types of ammunition including improvised projectiles.
        blunderbusses,
        /// Firearms that can take different types of cartridge-based ammunition.
        shotguns,

        assault_rifles,
        battle_rifles,
        sniper_rifles,

        light_machine_guns,
        heavy_machine_guns,

        // ---
        // Armor
        // ---

        /// Covers the top of the head.
        hats,
        /// Unarmored/lightly armored covering for the entire head.
        light_helmets,
        /// Armored covering for the entire head.
        heavy_helmets,

        /// One piece garments that cover the body and might cover the head.
        body_suits,

        /// Sleeveless garments that cover the torso.
        vests,
        /// Sleeved garments that cover the torso.
        shirts,
        /// Armored garments that cover the torso.
        cuirasses,

        /// Garments that cover only the crotch.
        thongs,
        /// Unarmored pants.
        pants,
        /// Armored pants.
        chausses,

        /// Flip flops, shower shoes, etc.
        open_toed_shoes,
        /// Sneakers, dress shoes, etc.
        shoes,
        /// Combat boots, armored shoes, etc.
        boots,

        /// Long, sleeveless garments that wraps around the wearer.
        cloaks,
        /// Long garment that hangs on the back.
        capes,

        /// Coverings for the shoulder.
        pauldrons,
        /// Coverings for the wrist.
        bracers,
        /// Coverings for the hand.
        gloves
    }

    Resistances: u64 {
        dry_resist,
        wet_resist,

        heat_resist,
        cold_resist,

        noise_resist,
        silence_resist,

        asphyxiate_resist,

        sticky_resist,

        crush_resist,
        pierce_resist,
        slash_resist,

        fear_resist,
        panic_resist,
        persuade_resist
    }

    Interactions: u64 {
        idling,
        dying,
        sleeping,

        talking,
        fighting,
        fleeing
    }

    Combat: u64 {
        dodging,

        blocking,
        pushing,
        /// Throwing opponents.
        throwing,

        crushing,
        stabbing,
        slashing,

        /// Throwing weapons like grenades, spears, shields, etc.
        projectile_throwing,
        unaimed_shooting,
        aimed_shooting
    }
}
