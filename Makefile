.PHONY: all cu

all: cu

cu:
	@echo "quasar" 
	@cd quasar && quasar build > /dev/null 2>&1 && cargo test -- --nocapture 2>&1 | grep -E "initialize CU|insert CU|verify CU"
	@echo ""
	@echo "anchor" 
	@cd anchor && anchor build > /dev/null 2>&1 && cargo test -- --nocapture 2>&1 | grep -E "initialize CU|insert CU|verify CU"
