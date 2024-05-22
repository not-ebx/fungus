use std::collections::HashMap;
use std::io::Error;
use log::info;
use rust_nx::nx_file::NXFile;
use rust_nx::nx_node::NXNode;
use fungus_utils::constants::game_constants::NX_FILES_DIR;
use fungus_utils::enums::inv_type::InvType;
use fungus_utils::fg_printc_info;
use fungus_utils::utility::item_utility::is_equipment;
use crate::game_data::game_info::item_info::ItemInfo;
use crate::entities::equipment::Equipment;
use crate::entities::item::Item;

pub struct ItemData {
    pub equipments: HashMap<i32, Equipment>,
    pub item_info: HashMap<i32, Item>
}

impl ItemData {
    pub fn new() -> Self {
        ItemData {
            equipments: Default::default(),
            item_info: Default::default(),
        }
    }

    pub fn fetch_equipment(&self, id: i32) -> Option<Equipment>{
        let id_c = id.clone();
        match self.equipments.get(&id_c) {
            Some(eqp) => Some(eqp.clone()),
            None => None
        }
    }

    pub fn fetch_item(&self, id: i32) -> Option<Item>{
        let id_c = id.clone();
        match self.item_info.get(&id) {
            Some(eqp) => Some(eqp.clone()),
            None => None
        }
    }

    pub fn load_all(&mut self) -> Result<(), Error>{
        self.load_items()?;
        self.load_equips()?;

        Ok(())
    }

    fn load_equips(&mut self) -> Result<(), Error>{
        fg_printc_info!("Loading Equipments");
        let character_nx_loc = NX_FILES_DIR.to_string() + "/Character.nx";
        let character_nx: NXFile = NXFile::new(&*character_nx_loc)?;
        let equip_categories = [
            "Accessory", "Android", "Cap", "Cape", "Coat", "Dragon", "Face", "Glove",
            "Longcoat", "Mechanic", "Pants", "PetEquip", "Ring", "Shield", "Shoes", "Totem", "Weapon", "MonsterBook"
        ];

        let categories_nodes: Vec<Option<&NXNode>> = equip_categories.iter().map(
            |&cat| character_nx.resolve(cat)
        ).collect();

        for node in categories_nodes.iter().filter_map(|&x| x) {
            if !node.has_children() {
                continue;
            }
            let children_nodes = character_nx.get_node_children(node);
            for &item_node in children_nodes.iter() {
                let name = item_node.name.clone();
                if name.len() > 0 {
                    let mut new_equip: Equipment;
                    if let Ok(item_id) = name.clone().replace(".img", "").parse::<i32>() {
                        new_equip = Equipment::new_default(item_id);
                    } else {
                        continue;
                    }
                    // Now load the data for the equipment.
                    let info_node_op = character_nx.get_node_child(item_node, "info");
                    if info_node_op.is_none() {
                        continue
                    }
                    let info_node = info_node_op.unwrap();
                    for item_info in character_nx.get_node_children(info_node) {
                        let node_name = item_info.name.clone();
                        let value = item_info.data.clone();
                        match node_name.as_str() {
                            "islot" => new_equip.i_slot = value.into(),
                            "vslot" => new_equip.v_slot = value.into(),
                            "reqJob" => new_equip.req_job = value.into(),
                            "reqLevel" => new_equip.req_level = value.into(),
                            "reqSTR" => new_equip.req_str = value.into(),
                            "reqDEX" => new_equip.req_dex = value.into(),
                            "reqINT" => new_equip.req_int = value.into(),
                            "reqLUK" => new_equip.req_luk = value.into(),
                            "reqPOP" => new_equip.req_pop = value.into(),
                            "incSTR" => new_equip.inc_str = value.into(),
                            "incDEX" => new_equip.inc_dex = value.into(),
                            "incINT" => new_equip.inc_int = value.into(),
                            "incLUK" => new_equip.inc_luk = value.into(),
                            "incPDD" => new_equip.inc_pdd = value.into(),
                            "incMDD" => new_equip.inc_mdd = value.into(),
                            "incMHP" => new_equip.inc_hp = value.into(),
                            "incMMP" => new_equip.inc_mp = value.into(),
                            "incPAD" => new_equip.inc_pad = value.into(),
                            "incMAD" => new_equip.inc_mad = value.into(),
                            "incEVA" => new_equip.inc_evasion = value.into(),
                            "incACC" => new_equip.inc_accuracy = value.into(),
                            "incSpeed" => new_equip.inc_speed = value.into(),
                            "incJump" => new_equip.inc_jump = value.into(),
                            "damR" => new_equip.inc_total_damage = value.into(),
                            "statR" => new_equip.attribute = value.into(),
                            "imdR" => new_equip.inc_ied = value.into(),
                            "bdR" => new_equip.inc_boss_damage_range = value.into(),
                            "tuc" => new_equip.total_upgrade_count = value.into(),
                            "IUCMax" => new_equip.inc_upgrade_count = value.into(),
                            "setItemID" => new_equip.set_id = value.into(),
                            "price" => new_equip.price = value.into(),
                            "attackSpeed" => new_equip.attack_speed = value.into(),
                            "cash" => new_equip.item.is_cash = value.into(),
                            "expireOnLogout" => new_equip.is_expired_on_logout = value.into(),
                            //"exItem" => new_equip.item.,
                            "notSale" => {
                                let val: bool = value.into();
                                new_equip.is_sellable = !val;
                            },
                            "only" => new_equip.is_unique = value.into(),
                            "tradeBlock" => new_equip.is_trade_blocked = value.into(),
                            "fixedPotential" => new_equip.has_fixed_potential = value.into(),
                            "noPotential" => {
                                let val: bool = value.into();
                                new_equip.is_potable = val;
                            },
                            "bossReward" => new_equip.is_boss_reward = value.into(),
                            "superiorEqp" => new_equip.is_superior_equip = value.into(),
                            "reduceReq" => new_equip.inc_reduce_req = value.into(),
                            //"fixedGrade" => new_equip.,
                            //"specialGrade" => new_equip.gra,
                            "charmEXP" => new_equip.charm_exp = value.into(),
                            "android" => new_equip.android = value.into(),
                            "grade" => new_equip.android_grade = value.into(),
                            _ => {}, // Handle unknown cases
                        }

                    }
                    // end of info
                    self.equipments.insert(new_equip.item.item_id.clone(), new_equip);
                }
            }
        }
        fg_printc_info!("Successfully loaded {} equips", self.equipments.len());
        Ok(())
    }

    fn load_items(&mut self) -> Result<(), Error> {
        fg_printc_info!("Loading Item Infos");
        let item_nx_loc = NX_FILES_DIR.to_string() + "/Item.nx";
        let item_nx: NXFile = NXFile::new(&*item_nx_loc)?;
        let item_categories = [
            "Cash", "Consume", "Etc", "Install", "Special"
        ];

        let categories_nodes: Vec<Option<&NXNode>> = item_categories.iter().map(
            |&cat| item_nx.resolve(cat)
        ).into_iter().collect();

        for node in categories_nodes.iter().filter_map(|&x| x) {
            let prefix_nodes = item_nx.get_node_children(node);
            for &prefix in prefix_nodes.iter() {
                for item_node in item_nx.get_node_children(prefix) {
                    let node_name = item_node.name.clone();
                    let mut new_item: Item;
                    if let Ok(item_id) = node_name.parse::<i32>() {
                        new_item = Item::default();
                        new_item.item_id = item_id;
                    } else {
                        continue;
                    }
                    // Now get the info node xd
                    let info_node_op = item_nx.get_node_child(item_node, "info");
                    if info_node_op.is_none() {
                        continue;
                    }
                    let info_node = info_node_op.unwrap();
                    for info in item_nx.get_node_children(info_node) {
                        let info_name = info.name.clone();
                        let value = info.data.clone();
                        // TODO finish this shit lol
                        match info_name.as_str() {
                            //"price" => new_item.price = value.into(),
                            //"slotMax" => new_item.max_slot = value.into(),
                            _ => {}
                        }
                    }
                    // end of info
                    // get the category
                    let item_inv = InvType::from(node.name.clone().as_str());
                    new_item.inv_type = item_inv;
                    self.item_info.insert(new_item.item_id.clone(), new_item);
                }
            }
        }
        fg_printc_info!("Successfully loaded {} items", self.item_info.len());
        Ok(())
    }

    pub fn get_inv_type(&self, id: i32) -> InvType {
        if(is_equipment(id)) {
            InvType::Equip
        } else {
            let item_info = self.item_info.get(&id);
            match item_info {
                None => {
                    InvType::None
                }
                Some(ii) => {
                    ii.inv_type.clone()
                }
            }
        }
    }
}