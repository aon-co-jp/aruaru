#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsuranceCategory {
    Life,
    Medical,
    Cancer,
    DisabilityIncome,
    Auto,
    FireAndHome,
    Liability,
    Travel,
    Pet,
    Business,
    Cyber,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsuranceEvidenceClass {
    RegulatorOrPublicAgency,
    OfficialInsurerDocument,
    PublicRankingOrSurvey,
    BrokerComparison,
    ReviewOrClaimExperience,
    AdvertisementOrCreatorClaim,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InsuranceTop100Item {
    pub rank: usize,
    pub category: InsuranceCategory,
    pub topic: &'static str,
    pub why_useful_or_popular: &'static str,
    pub features: &'static str,
    pub merits: &'static str,
    pub demerits: &'static str,
    pub check_points: &'static str,
    pub evidence: InsuranceEvidenceClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InsuranceCrawlerPolicy {
    pub schedule_local_time: &'static str,
    pub source_priority: Vec<&'static str>,
    pub safety_rules: Vec<&'static str>,
    pub output_files: Vec<&'static str>,
}

pub fn daily_insurance_top100_crawl_policy() -> InsuranceCrawlerPolicy {
    InsuranceCrawlerPolicy {
        schedule_local_time: "04:15 Asia/Tokyo daily",
        source_priority: vec![
            "financial regulators, consumer agencies and public insurance guidance",
            "official insurer product documents and important matter explanations",
            "public rankings, market surveys and claims statistics",
            "broker comparison pages with commission/conflict labels",
            "user reviews, claim experiences and SNS posts with confidence labels",
            "YouTube and advertisement claims, never treated as neutral facts",
        ],
        safety_rules: vec![
            "do not provide personalized insurance, legal, tax or investment advice without licensed review",
            "separate premium, coverage, exclusions, waiting period, surrender value and claim conditions",
            "mark advertisements, affiliate pages and creator recommendations as conflict-of-interest possible",
            "show who may benefit, who may not need it, and what public insurance already covers",
            "store source URL, access date, product revision date, region and confidence score",
        ],
        output_files: vec![
            "data/insurance_top100/latest.json",
            "data/insurance_top100/history/YYYY-MM-DD.json",
            "data/insurance_top100/diff/YYYY-MM-DD.md",
            "reports/insurance_top100.md",
            "reports/insurance_top100_redmine.md",
        ],
    }
}

fn insurance_item(rank: usize, category: InsuranceCategory, topic: &'static str) -> InsuranceTop100Item {
    InsuranceTop100Item {
        rank,
        category,
        topic,
        why_useful_or_popular: "popular because it reduces large unexpected risks, supports family security, satisfies loan/business requirements, or is heavily advertised",
        features: "tracks premium, coverage, exclusions, claim conditions, waiting periods, renewal rules and public-insurance overlap",
        merits: "helps users learn what to compare before spending money and helps aruaru-ai generate safer insurance comparison prompts",
        demerits: "terms differ by age, region, health status and contract revision; advertisements may overemphasize benefits",
        check_points: "confirm coverage limit, exclusions, waiting period, claim documents, cancellation rules, premium increase and conflict of interest",
        evidence: InsuranceEvidenceClass::PublicRankingOrSurvey,
    }
}

pub fn insurance_top100_seed_catalog() -> Vec<InsuranceTop100Item> {
    let seeds: [(InsuranceCategory, &str); 100] = [
        (InsuranceCategory::Life, "Term life insurance"),
        (InsuranceCategory::Life, "Whole life insurance"),
        (InsuranceCategory::Life, "Income protection for family breadwinner"),
        (InsuranceCategory::Life, "Mortgage-linked life coverage"),
        (InsuranceCategory::Life, "Funeral cost coverage"),
        (InsuranceCategory::Life, "Education fund protection"),
        (InsuranceCategory::Life, "Single-premium life policy risk watch"),
        (InsuranceCategory::Life, "Foreign-currency life insurance risk watch"),
        (InsuranceCategory::Life, "Low-cost online life insurance"),
        (InsuranceCategory::Life, "Group life insurance through employer"),
        (InsuranceCategory::Medical, "Hospitalization insurance"),
        (InsuranceCategory::Medical, "Surgery benefit insurance"),
        (InsuranceCategory::Medical, "Advanced medical treatment rider"),
        (InsuranceCategory::Medical, "Outpatient treatment rider"),
        (InsuranceCategory::Medical, "Women-specific medical insurance"),
        (InsuranceCategory::Medical, "Senior medical insurance"),
        (InsuranceCategory::Medical, "Short-term medical cost coverage"),
        (InsuranceCategory::Medical, "High-deductible medical plan comparison"),
        (InsuranceCategory::Medical, "Telemedicine-linked insurance service"),
        (InsuranceCategory::Medical, "Public health insurance overlap guide"),
        (InsuranceCategory::Cancer, "Cancer diagnosis lump-sum benefit"),
        (InsuranceCategory::Cancer, "Cancer treatment monthly benefit"),
        (InsuranceCategory::Cancer, "Cancer outpatient chemotherapy coverage"),
        (InsuranceCategory::Cancer, "Cancer recurrence coverage"),
        (InsuranceCategory::Cancer, "Cancer genomic medicine rider"),
        (InsuranceCategory::Cancer, "Cancer income support"),
        (InsuranceCategory::Cancer, "Cancer insurance waiting period watch"),
        (InsuranceCategory::Cancer, "Cancer survivor re-entry plans"),
        (InsuranceCategory::Cancer, "Cancer family history comparison"),
        (InsuranceCategory::Cancer, "Cancer insurance advertisement claim watch"),
        (InsuranceCategory::DisabilityIncome, "Disability income insurance"),
        (InsuranceCategory::DisabilityIncome, "Long-term care insurance"),
        (InsuranceCategory::DisabilityIncome, "Dementia insurance"),
        (InsuranceCategory::DisabilityIncome, "Nursing care lump-sum benefit"),
        (InsuranceCategory::DisabilityIncome, "Mental health disability coverage watch"),
        (InsuranceCategory::DisabilityIncome, "Self-employed income protection"),
        (InsuranceCategory::DisabilityIncome, "Accident disability coverage"),
        (InsuranceCategory::DisabilityIncome, "Critical illness income support"),
        (InsuranceCategory::DisabilityIncome, "Employer sick-pay gap guide"),
        (InsuranceCategory::DisabilityIncome, "Public pension disability overlap guide"),
        (InsuranceCategory::Auto, "Mandatory auto liability insurance"),
        (InsuranceCategory::Auto, "Voluntary auto insurance"),
        (InsuranceCategory::Auto, "Bodily injury liability"),
        (InsuranceCategory::Auto, "Property damage liability"),
        (InsuranceCategory::Auto, "Vehicle damage coverage"),
        (InsuranceCategory::Auto, "No-fault personal injury coverage"),
        (InsuranceCategory::Auto, "Roadside assistance"),
        (InsuranceCategory::Auto, "Dashcam discount and telematics"),
        (InsuranceCategory::Auto, "Classic car insurance"),
        (InsuranceCategory::Auto, "EV insurance and battery risk"),
        (InsuranceCategory::FireAndHome, "Fire insurance"),
        (InsuranceCategory::FireAndHome, "Earthquake insurance"),
        (InsuranceCategory::FireAndHome, "Flood and water damage coverage"),
        (InsuranceCategory::FireAndHome, "Theft and burglary coverage"),
        (InsuranceCategory::FireAndHome, "Home equipment breakdown"),
        (InsuranceCategory::FireAndHome, "Condominium shared-area coverage"),
        (InsuranceCategory::FireAndHome, "Rental tenant liability"),
        (InsuranceCategory::FireAndHome, "Solar panel and storage battery coverage"),
        (InsuranceCategory::FireAndHome, "Home theater equipment coverage"),
        (InsuranceCategory::FireAndHome, "Construction and renovation insurance"),
        (InsuranceCategory::Liability, "Personal liability insurance"),
        (InsuranceCategory::Liability, "Bicycle liability insurance"),
        (InsuranceCategory::Liability, "Dog owner liability"),
        (InsuranceCategory::Liability, "Tenant liability"),
        (InsuranceCategory::Liability, "Professional liability"),
        (InsuranceCategory::Liability, "Product liability"),
        (InsuranceCategory::Liability, "Event liability"),
        (InsuranceCategory::Liability, "Director and officer liability"),
        (InsuranceCategory::Liability, "Drone liability"),
        (InsuranceCategory::Liability, "Creator and influencer liability"),
        (InsuranceCategory::Travel, "Overseas travel insurance"),
        (InsuranceCategory::Travel, "Domestic travel insurance"),
        (InsuranceCategory::Travel, "Flight cancellation coverage"),
        (InsuranceCategory::Travel, "Lost baggage coverage"),
        (InsuranceCategory::Travel, "Medical evacuation coverage"),
        (InsuranceCategory::Travel, "Credit card included insurance"),
        (InsuranceCategory::Travel, "Study abroad insurance"),
        (InsuranceCategory::Travel, "Digital nomad insurance"),
        (InsuranceCategory::Travel, "Rental car excess coverage"),
        (InsuranceCategory::Travel, "Travel insurance exclusion watch"),
        (InsuranceCategory::Pet, "Pet medical insurance"),
        (InsuranceCategory::Pet, "Pet surgery coverage"),
        (InsuranceCategory::Pet, "Pet chronic disease coverage"),
        (InsuranceCategory::Pet, "Pet age-limit comparison"),
        (InsuranceCategory::Pet, "Pet breed-specific risk watch"),
        (InsuranceCategory::Business, "Small business property insurance"),
        (InsuranceCategory::Business, "Business interruption insurance"),
        (InsuranceCategory::Business, "Workers accident insurance"),
        (InsuranceCategory::Business, "Construction contractor insurance"),
        (InsuranceCategory::Business, "Clinic and telemedicine business insurance"),
        (InsuranceCategory::Business, "Data center facility insurance"),
        (InsuranceCategory::Cyber, "Cyber insurance"),
        (InsuranceCategory::Cyber, "Ransomware response coverage"),
        (InsuranceCategory::Cyber, "Data breach liability"),
        (InsuranceCategory::Cyber, "Cloud outage coverage"),
        (InsuranceCategory::Cyber, "AI service liability insurance"),
        (InsuranceCategory::Cyber, "E-commerce fraud insurance"),
    ];

    seeds
        .iter()
        .enumerate()
        .map(|(index, (category, topic))| insurance_item(index + 1, *category, topic))
        .collect()
}

pub fn quality_gate_smoke_check() -> usize {
    let policy = daily_insurance_top100_crawl_policy();
    let catalog = insurance_top100_seed_catalog();
    let categories = [
        InsuranceCategory::Life,
        InsuranceCategory::Medical,
        InsuranceCategory::Cancer,
        InsuranceCategory::DisabilityIncome,
        InsuranceCategory::Auto,
        InsuranceCategory::FireAndHome,
        InsuranceCategory::Liability,
        InsuranceCategory::Travel,
        InsuranceCategory::Pet,
        InsuranceCategory::Business,
        InsuranceCategory::Cyber,
    ];
    let evidence = [
        InsuranceEvidenceClass::RegulatorOrPublicAgency,
        InsuranceEvidenceClass::OfficialInsurerDocument,
        InsuranceEvidenceClass::PublicRankingOrSurvey,
        InsuranceEvidenceClass::BrokerComparison,
        InsuranceEvidenceClass::ReviewOrClaimExperience,
        InsuranceEvidenceClass::AdvertisementOrCreatorClaim,
    ];

    assert!(policy.schedule_local_time.contains("Asia/Tokyo"));
    assert_eq!(catalog.len(), 100);
    catalog.len() + categories.len() + evidence.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insurance_catalog_has_exactly_top100_items() {
        let catalog = insurance_top100_seed_catalog();
        assert_eq!(catalog.len(), 100);
        assert_eq!(catalog.first().unwrap().rank, 1);
        assert_eq!(catalog.last().unwrap().rank, 100);
    }

    #[test]
    fn insurance_policy_separates_ads_from_facts() {
        let policy = daily_insurance_top100_crawl_policy();
        assert!(policy.safety_rules.iter().any(|rule| rule.contains("advertisements")));
        assert!(policy.safety_rules.iter().any(|rule| rule.contains("personalized insurance")));
    }

    #[test]
    fn insurance_catalog_includes_required_topics() {
        let topics: Vec<&str> = insurance_top100_seed_catalog().into_iter().map(|item| item.topic).collect();
        assert!(topics.contains(&"Cyber insurance"));
        assert!(topics.contains(&"Earthquake insurance"));
        assert!(topics.contains(&"EV insurance and battery risk"));
        assert!(topics.contains(&"Home theater equipment coverage"));
        assert!(topics.contains(&"AI service liability insurance"));
    }
}
