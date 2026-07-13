---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-corvid-asa
artifact: research
---

# Research

The existing Fledge lane checks only three file names. The repository has no native build or tests, so deterministic content-manifest validation is the appropriate blocking lane. The expanded lane confirms all HTML, CSS, robots, sitemap, and favicon artifacts exist and are non-empty.
