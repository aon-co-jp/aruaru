#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarEvidenceClass {
    OfficialSalesData,
    ManufacturerSpec,
    UsedMarketSignal,
    ReviewOrOwnerReport,
    HistoricalLegacy,
    RiskOrRecallSignal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarMarketScope {
    JapanNewCar,
    WorldNewCar,
    UsedMarket,
    HistoricalClassic,
    CommercialOrKei,
    EvHybrid,
    LuxuryPerformance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarTop100Item {
    pub rank: usize,
    pub name: &'static str,
    pub maker: &'static str,
    pub scope: CarMarketScope,
    pub why_popular_or_selling: &'static str,
    pub features: &'static str,
    pub representative_specs: &'static str,
    pub merits: &'static str,
    pub demerits: &'static str,
    pub evidence: CarEvidenceClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarCrawlerPolicy {
    pub schedule_local_time: &'static str,
    pub source_priority: Vec<&'static str>,
    pub safety_rules: Vec<&'static str>,
    pub output_files: Vec<&'static str>,
}

pub fn daily_car_top100_crawl_policy() -> CarCrawlerPolicy {
    CarCrawlerPolicy {
        schedule_local_time: "04:05 Asia/Tokyo daily",
        source_priority: vec![
            "official sales reports and registration statistics",
            "manufacturer official specifications and recall pages",
            "used car marketplace trend signals",
            "owner reviews, repair reports and insurance cost signals",
            "reputable automotive journalism and long-term road tests",
            "YouTube test drives and creator opinions, never treated as verified facts",
        ],
        safety_rules: vec![
            "separate Japan popularity, world popularity, used-market popularity and historical legacy",
            "separate official specifications from owner reviews and creator claims",
            "show maintenance cost, insurance cost, recall, theft risk and resale-value warnings",
            "do not present old classic popularity as current sales performance",
            "store source URL, access date, market region, model year and confidence score",
        ],
        output_files: vec![
            "data/car_top100/latest.json",
            "data/car_top100/history/YYYY-MM-DD.json",
            "data/car_top100/diff/YYYY-MM-DD.md",
            "reports/car_top100.md",
            "reports/car_top100_redmine.md",
        ],
    }
}

fn car_item(rank: usize, name: &'static str, maker: &'static str, scope: CarMarketScope) -> CarTop100Item {
    CarTop100Item {
        rank,
        name,
        maker,
        scope,
        why_popular_or_selling: "price, reliability, brand strength, practical size, fuel economy, repairability, resale value or emotional legacy",
        features: "tracks body type, model generation, powertrain, safety equipment, market region, owner reputation and sales momentum",
        representative_specs: "engine or motor class, drivetrain, size class, passenger/cargo use, fuel/electric efficiency and price band should be refreshed by crawler",
        merits: "helps aruaru-ai connect car popularity to real needs such as family use, business use, EV trend, luxury demand and used-market value",
        demerits: "rankings change by year and region; specs differ by model year and trim; reviews may be subjective",
        evidence: CarEvidenceClass::UsedMarketSignal,
    }
}

pub fn car_top100_seed_catalog() -> Vec<CarTop100Item> {
    let seeds: [(&str, &str, CarMarketScope); 100] = [
        ("Toyota Corolla", "Toyota", CarMarketScope::WorldNewCar),
        ("Toyota RAV4", "Toyota", CarMarketScope::WorldNewCar),
        ("Tesla Model Y", "Tesla", CarMarketScope::EvHybrid),
        ("Ford F-Series", "Ford", CarMarketScope::WorldNewCar),
        ("Toyota Camry", "Toyota", CarMarketScope::WorldNewCar),
        ("Honda CR-V", "Honda", CarMarketScope::WorldNewCar),
        ("Honda Civic", "Honda", CarMarketScope::WorldNewCar),
        ("Toyota Yaris", "Toyota", CarMarketScope::JapanNewCar),
        ("Toyota Prius", "Toyota", CarMarketScope::EvHybrid),
        ("Toyota Land Cruiser", "Toyota", CarMarketScope::UsedMarket),
        ("Honda N-BOX", "Honda", CarMarketScope::CommercialOrKei),
        ("Suzuki Jimny", "Suzuki", CarMarketScope::CommercialOrKei),
        ("Toyota Alphard", "Toyota", CarMarketScope::LuxuryPerformance),
        ("Toyota Sienta", "Toyota", CarMarketScope::JapanNewCar),
        ("Toyota Aqua", "Toyota", CarMarketScope::JapanNewCar),
        ("Nissan Note", "Nissan", CarMarketScope::JapanNewCar),
        ("Daihatsu Tanto", "Daihatsu", CarMarketScope::CommercialOrKei),
        ("Suzuki Spacia", "Suzuki", CarMarketScope::CommercialOrKei),
        ("Toyota HiAce", "Toyota", CarMarketScope::CommercialOrKei),
        ("Honda Fit", "Honda", CarMarketScope::JapanNewCar),
        ("Mazda MX-5 Roadster", "Mazda", CarMarketScope::HistoricalClassic),
        ("Nissan Skyline GT-R", "Nissan", CarMarketScope::HistoricalClassic),
        ("Toyota Supra", "Toyota", CarMarketScope::HistoricalClassic),
        ("Porsche 911", "Porsche", CarMarketScope::LuxuryPerformance),
        ("Volkswagen Golf", "Volkswagen", CarMarketScope::WorldNewCar),
        ("Volkswagen Beetle", "Volkswagen", CarMarketScope::HistoricalClassic),
        ("Mercedes-Benz S-Class", "Mercedes-Benz", CarMarketScope::LuxuryPerformance),
        ("BMW 3 Series", "BMW", CarMarketScope::LuxuryPerformance),
        ("Jeep Wrangler", "Jeep", CarMarketScope::UsedMarket),
        ("Mini Cooper", "Mini", CarMarketScope::HistoricalClassic),
        ("Tesla Model 3", "Tesla", CarMarketScope::EvHybrid),
        ("Nissan Leaf", "Nissan", CarMarketScope::EvHybrid),
        ("Hyundai Ioniq 5", "Hyundai", CarMarketScope::EvHybrid),
        ("BYD Seal", "BYD", CarMarketScope::EvHybrid),
        ("BYD Dolphin", "BYD", CarMarketScope::EvHybrid),
        ("Ford Mustang", "Ford", CarMarketScope::HistoricalClassic),
        ("Chevrolet Corvette", "Chevrolet", CarMarketScope::HistoricalClassic),
        ("Chevrolet Silverado", "Chevrolet", CarMarketScope::WorldNewCar),
        ("Ram Pickup", "Ram", CarMarketScope::WorldNewCar),
        ("Honda Accord", "Honda", CarMarketScope::WorldNewCar),
        ("Subaru Forester", "Subaru", CarMarketScope::JapanNewCar),
        ("Subaru Outback", "Subaru", CarMarketScope::WorldNewCar),
        ("Subaru Impreza", "Subaru", CarMarketScope::HistoricalClassic),
        ("Mitsubishi Delica D:5", "Mitsubishi", CarMarketScope::UsedMarket),
        ("Mitsubishi Pajero", "Mitsubishi", CarMarketScope::HistoricalClassic),
        ("Lexus RX", "Lexus", CarMarketScope::LuxuryPerformance),
        ("Lexus LS", "Lexus", CarMarketScope::LuxuryPerformance),
        ("Lexus LC", "Lexus", CarMarketScope::LuxuryPerformance),
        ("Toyota Crown", "Toyota", CarMarketScope::HistoricalClassic),
        ("Toyota Harrier", "Toyota", CarMarketScope::JapanNewCar),
        ("Toyota Noah", "Toyota", CarMarketScope::JapanNewCar),
        ("Toyota Voxy", "Toyota", CarMarketScope::JapanNewCar),
        ("Nissan Serena", "Nissan", CarMarketScope::JapanNewCar),
        ("Honda Step WGN", "Honda", CarMarketScope::JapanNewCar),
        ("Honda Odyssey", "Honda", CarMarketScope::UsedMarket),
        ("Nissan GT-R", "Nissan", CarMarketScope::LuxuryPerformance),
        ("Mazda CX-5", "Mazda", CarMarketScope::JapanNewCar),
        ("Mazda CX-8", "Mazda", CarMarketScope::JapanNewCar),
        ("Mazda3", "Mazda", CarMarketScope::WorldNewCar),
        ("Suzuki Swift", "Suzuki", CarMarketScope::JapanNewCar),
        ("Suzuki Wagon R", "Suzuki", CarMarketScope::CommercialOrKei),
        ("Suzuki Alto", "Suzuki", CarMarketScope::CommercialOrKei),
        ("Daihatsu Move", "Daihatsu", CarMarketScope::CommercialOrKei),
        ("Daihatsu Mira", "Daihatsu", CarMarketScope::CommercialOrKei),
        ("Honda S660", "Honda", CarMarketScope::HistoricalClassic),
        ("Toyota GR86", "Toyota", CarMarketScope::LuxuryPerformance),
        ("Subaru BRZ", "Subaru", CarMarketScope::LuxuryPerformance),
        ("Hyundai Tucson", "Hyundai", CarMarketScope::WorldNewCar),
        ("Kia Sportage", "Kia", CarMarketScope::WorldNewCar),
        ("Kia Telluride", "Kia", CarMarketScope::WorldNewCar),
        ("Audi A4", "Audi", CarMarketScope::LuxuryPerformance),
        ("Audi Q5", "Audi", CarMarketScope::LuxuryPerformance),
        ("Mercedes-Benz G-Class", "Mercedes-Benz", CarMarketScope::LuxuryPerformance),
        ("BMW X5", "BMW", CarMarketScope::LuxuryPerformance),
        ("Range Rover", "Land Rover", CarMarketScope::LuxuryPerformance),
        ("Volvo XC60", "Volvo", CarMarketScope::LuxuryPerformance),
        ("Peugeot 208", "Peugeot", CarMarketScope::WorldNewCar),
        ("Renault Clio", "Renault", CarMarketScope::WorldNewCar),
        ("Fiat 500", "Fiat", CarMarketScope::HistoricalClassic),
        ("Citroen 2CV", "Citroen", CarMarketScope::HistoricalClassic),
        ("Lamborghini Countach", "Lamborghini", CarMarketScope::HistoricalClassic),
        ("Ferrari F40", "Ferrari", CarMarketScope::HistoricalClassic),
        ("Ferrari 911 rival catalog", "Ferrari", CarMarketScope::LuxuryPerformance),
        ("McLaren F1", "McLaren", CarMarketScope::HistoricalClassic),
        ("Bugatti Veyron", "Bugatti", CarMarketScope::HistoricalClassic),
        ("Bugatti Chiron", "Bugatti", CarMarketScope::LuxuryPerformance),
        ("Koenigsegg Jesko", "Koenigsegg", CarMarketScope::LuxuryPerformance),
        ("Rivian R1T", "Rivian", CarMarketScope::EvHybrid),
        ("Lucid Air", "Lucid", CarMarketScope::EvHybrid),
        ("Toyota Hilux", "Toyota", CarMarketScope::WorldNewCar),
        ("Isuzu D-Max", "Isuzu", CarMarketScope::WorldNewCar),
        ("Mercedes-Benz Sprinter", "Mercedes-Benz", CarMarketScope::CommercialOrKei),
        ("Ford Transit", "Ford", CarMarketScope::CommercialOrKei),
        ("Toyota Probox", "Toyota", CarMarketScope::CommercialOrKei),
        ("Nissan AD", "Nissan", CarMarketScope::CommercialOrKei),
        ("Honda Super Cub adjacent mobility", "Honda", CarMarketScope::HistoricalClassic),
        ("Toyota Century", "Toyota", CarMarketScope::LuxuryPerformance),
        ("Rolls-Royce Phantom", "Rolls-Royce", CarMarketScope::LuxuryPerformance),
        ("Bentley Continental GT", "Bentley", CarMarketScope::LuxuryPerformance),
        ("Aston Martin DB series", "Aston Martin", CarMarketScope::HistoricalClassic),
    ];

    seeds
        .iter()
        .enumerate()
        .map(|(index, (name, maker, scope))| car_item(index + 1, name, maker, *scope))
        .collect()
}

pub fn quality_gate_smoke_check() -> usize {
    let policy = daily_car_top100_crawl_policy();
    let catalog = car_top100_seed_catalog();
    let evidence_classes = [
        CarEvidenceClass::OfficialSalesData,
        CarEvidenceClass::ManufacturerSpec,
        CarEvidenceClass::UsedMarketSignal,
        CarEvidenceClass::ReviewOrOwnerReport,
        CarEvidenceClass::HistoricalLegacy,
        CarEvidenceClass::RiskOrRecallSignal,
    ];
    let scopes = [
        CarMarketScope::JapanNewCar,
        CarMarketScope::WorldNewCar,
        CarMarketScope::UsedMarket,
        CarMarketScope::HistoricalClassic,
        CarMarketScope::CommercialOrKei,
        CarMarketScope::EvHybrid,
        CarMarketScope::LuxuryPerformance,
    ];

    assert!(policy.schedule_local_time.contains("Asia/Tokyo"));
    assert_eq!(catalog.len(), 100);
    catalog.len() + evidence_classes.len() + scopes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn car_catalog_has_exactly_top100_items() {
        let catalog = car_top100_seed_catalog();
        assert_eq!(catalog.len(), 100);
        assert_eq!(catalog.first().unwrap().rank, 1);
        assert_eq!(catalog.last().unwrap().rank, 100);
    }

    #[test]
    fn car_policy_separates_sales_reviews_and_legacy() {
        let policy = daily_car_top100_crawl_policy();
        assert!(policy.safety_rules.iter().any(|rule| rule.contains("Japan popularity")));
        assert!(policy.safety_rules.iter().any(|rule| rule.contains("official specifications")));
    }

    #[test]
    fn car_catalog_includes_required_models() {
        let names: Vec<&str> = car_top100_seed_catalog().into_iter().map(|item| item.name).collect();
        assert!(names.contains(&"Toyota Corolla"));
        assert!(names.contains(&"Honda N-BOX"));
        assert!(names.contains(&"Tesla Model Y"));
        assert!(names.contains(&"Nissan Skyline GT-R"));
        assert!(names.contains(&"Suzuki Jimny"));
    }
}
