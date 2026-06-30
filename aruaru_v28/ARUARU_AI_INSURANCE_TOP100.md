# aruaru-ai Insurance TOP100 Crawler

## Problem
Insurance information is useful, but it is high-stakes and often influenced by advertising, affiliate commissions, broker incentives, user anecdotes and product revisions. aruaru-ai must not treat advertisements or creator recommendations as neutral facts.

## Policy
- Crawl daily at 04:15 Asia/Tokyo.
- Separate public/regulatory guidance, official insurer documents, rankings, broker comparisons, reviews and advertisements.
- Do not provide personalized insurance, legal, tax or investment advice without licensed review.
- Always show coverage, exclusions, waiting periods, premium, claim rules, cancellation rules and public-insurance overlap.

## Output
- `data/insurance_top100/latest.json`
- `data/insurance_top100/history/YYYY-MM-DD.json`
- `data/insurance_top100/diff/YYYY-MM-DD.md`
- `reports/insurance_top100.md`
- `reports/insurance_top100_redmine.md`

## TOP100 categories
- life insurance
- medical insurance
- cancer insurance
- disability income / long-term care
- auto insurance
- fire/home/earthquake insurance
- liability insurance
- travel insurance
- pet insurance
- business insurance
- cyber insurance

## Required analysis fields
- product/topic name
- category
- why useful or popular
- features
- merits
- demerits
- who may benefit
- who may not need it
- public insurance overlap
- exclusions and waiting period
- advertisement/conflict-of-interest warning
- source confidence
