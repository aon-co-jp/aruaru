# aruaru-ai Car TOP100 Crawler

## Problem
Car popularity cannot be judged only by current new-car sales. aruaru-ai must separate Japan sales, world sales, used-market popularity, historical classics, kei/commercial cars, EV/hybrid trends, luxury demand, repairability, insurance cost, theft risk, recalls, resale value and owner reviews.

## Policy
- Crawl daily at 04:05 Asia/Tokyo.
- Separate Fact, Estimate, Review, Claim, Risk and Legacy.
- Do not mix current sales with old classic popularity.
- Always store region, model year, trim, access date and confidence.

## Output
- `data/car_top100/latest.json`
- `data/car_top100/history/YYYY-MM-DD.json`
- `data/car_top100/diff/YYYY-MM-DD.md`
- `reports/car_top100.md`
- `reports/car_top100_redmine.md`

## Required analysis fields
- car name
- maker
- region
- current sales popularity
- used-market popularity
- historical legacy
- why popular or selling
- features
- representative specs
- merits
- demerits
- maintenance cost
- insurance cost
- recall/theft warnings
- resale value
- source confidence
