use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Define a struct to hold selector configurations
struct SelectorConfig {
    selector: String,
    name: String, // To identify the type of content being extracted
}

fn main() -> Result<(), Box<dyn Error>> {
    let website_folder = "https://www.slimstock.com/blog";
    let pages = vec![
        "how-to-start-creating-your-csrd-report-the-importance-of-double-materiality-assessment/",
        "optimised-service-levels/",
        "expanding-supply-chain-planning/",
        "navigating-the-complexities-of-aftermarket-inventory-management/",
        "scor-model/",
        "management-by-exception/",
        "tariff-ic-strategies-for-your-business-planning/",
        "stock-control/",
        "supply-chain-management/",
        "operational-efficiency/",
        "how-to-overcome-forecasting-difficulties-in-the-fashion-and-textile-industry/",
        "capacity-planning/",
        "supply-chain-trends-for-2025/",
        "reverse-logistics/",
        "the-art-of-implementing-sop/",
        "cross-docking/",
        "purchasing-department/",
        "logistics-networks/",
        "days-inventory-outstanding/",
        "how-to-solve-wastage-problems/",
        "sustainable-supply-chain-practices-in-the-food-and-beverage-industry/",
        "pharmaceutical-supply-chain-management-in-the-mea-region/",
        "cash-conversion-cycle/",
        "inventory-management/",
        "corporate-sustainability-reporting-directive/",
        "long-tail-inventory/",
        "digital-supply-chain-investment/",
        "monthly-vs-weekly-forecasting/",
        "the-pareto-principle/",
        "8-inventory-management-video-tips-for-chinese-new-year/",
        "reasons-for-keeping-inventory/",
        "meio-supply-chain/",
        "supplier-collaboration/",
        "grocery-inventory-management/",
        "demand-management/",
        "sales-and-operations-planning-process/",
        "is-your-business-trapped-in-cash-flow-hell/",
        "food-inventory-management/",
        "10-different-types-of-stock-in-your-warehouse/",
        "whats-the-best-formula-to-achieve-optimal-stock-levels/",
        "working-capital/",
        "stock-replenishment/",
        "supply-chain-collaboration/",
        "inventory-allocation-perfect-balance/",
        "chief-supply-chain-officer-csco/",
        "supply-chain-centre-of-excellence/",
        "digital-twin/",
        "what-is-return-on-investment-roi/",
        "machine-learning/",
        "e2e-supply-chain-planning/",
        "inventory-cost/",
        "sop-supply-chain-roadmap/",
        "inventory-turnover/",
        "economic-order-quantity-eoq/",
        "supply-planning/",
        "sales-and-operations-planning-digital-age/",
        "safety-stock-inventory-qa/",
        "10-tips-for-managing-broad-product-ranges/",
        "how-to-solve-warehouse-space-shortage/",
        "inventory-optimization/",
        "how-e-commerce-is-impacting-inventory-management/",
        "bullwhip-effect-explained/",
        "master-data/",
        "product-lifecycle-management/",
        "supply-chain-kpis/",
        "otif-guide/",
        "forecast-accuracy/",
        "demand-planning/",
        "bill-of-materials-bom/",
        "assortment-planning/",
        "spreadsheets/",
        "abc-analysis/",
        "seasonal-demand-forecasting/",
        "improve-procurement-for-increased-customer-satisfaction/",
        "delivery-lead-time/",
        "product-availability/",
        "excess-stock/",
        "demand-forecasting/",
        "lean-logistics/",
        "reorder-point/",
        "production-planning/",
        "disruption-on-the-red-sea/",
        "overcome-bathroom-supply-chain-challenges/",
        "integrated-business-planning-ibp/",
        "just-in-time/",
        "category-management-in-retail/",
        "stockouts/",
        "schrodingers-inventory-paradox/",
        "sop-kpis/",
        "transform-your-sop-with-machine-learning/",
        "service-level/",
        "supplier-relationship-management/",
        "time-to-tackle-the-supply-chain-talent-crisis/",
        "quantifying-the-value-of-sop/",
        "vendor-managed-inventory/",
        "inventory-management-with-sop/",
        "business-intelligence/",
        "torque-interview-with-slimstock/",
        "pioneering-a-waste-free-supply-chain/",
        "demand-planner/",
        "leverage-sandop-for-competitive-advantage/",
        "supply-chain-sustainability/",
        "scenario-planning/",
        "appetite-for-supply-chain-risk/",
        "decoupling-point/",
        "dynamic-stock-replenishment-revolution/",
        "human-centric-supply-chain/",
        "manufacturing-supply-chain/",
        "digital-transformation/",
        "panama-canal-disruption/",
        "minimum-order-quantity/",
        "assortment-strategy/",
        "omnichannel-strategy/",
    ];

    // Define multiple selectors to extract different types of content
    let selectors = vec![
        SelectorConfig {
            selector: "p".to_string(),
            name: "paragraph".to_string(),
        },
        SelectorConfig {
            selector: "h1, h2, h3".to_string(),
            name: "heading".to_string(),
        },
        // Add more selectors as needed
    ];

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36")
        .build()?;

    let mut csv_output = String::from("Page,ContentType,Content\n"); // Updated CSV header

    for page in pages {
        let url = format!("{}/{}", website_folder, page);
        println!("Crawling URL: {}", url);

        // Process each selector for the page
        for selector_config in &selectors {
            match fetch_and_extract(&client, &url, &selector_config.selector) {
                Ok(result) => {
                    println!(
                        "Extracted {} content from {}: \n{}",
                        selector_config.name, page, result
                    );
                    for line in result.lines() {
                        let cleaned_line = line.trim();
                        if !cleaned_line.is_empty() {
                            csv_output.push_str(&format!(
                                "{},\"{}\",\"{}\"\n",
                                page,
                                selector_config.name,
                                cleaned_line.replace("\"", "\"\"")
                            ));
                        }
                    }
                }
                Err(err) => eprintln!(
                    "Failed to process {} content from {}: {}",
                    selector_config.name, page, err
                ),
            }
        }
    }

    // Write the CSV content to a file
    let mut file = File::create("output.csv")?;
    file.write_all(csv_output.as_bytes())?;
    println!("Content written to output.csv");

    Ok(())
}

fn fetch_and_extract(client: &Client, url: &str, selector: &str) -> Result<String, Box<dyn Error>> {
    // Fetch the page content
    let response = client.get(url).send()?.text()?;

    // Parse the HTML
    let document = Html::parse_document(&response);

    // Build the selector
    let css_selector =
        Selector::parse(selector).map_err(|e| format!("Failed to parse CSS selector: {:?}", e))?;

    // Extract matching elements
    let mut extracted_content = String::new();
    for element in document.select(&css_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            extracted_content.push_str(&text);
            extracted_content.push('\n');
        }
    }

    if extracted_content.is_empty() {
        return Err("No content found for the given selector".into());
    }

    Ok(extracted_content)
}
