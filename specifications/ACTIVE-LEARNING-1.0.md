# Active Learning 1.0

Active Learning runs controlled sessions with a task, environment, robot profile, allowed skills, success criterion, attempt budget, duration budget, energy budget, seed, and validation stage. It classifies failures and generates candidate improvements inside an externally defined safety envelope.

Learning cannot weaken Safety Governor limits. Learned behavior follows `EXPERIMENT → CANDIDATE → SIM_VALIDATED → HIL_VALIDATED → HARDWARE_VALIDATED → PRODUCTION`; it never promotes automatically to production.
