.PHONY: all cu diff

all: cu diff

diff:
	@quasar_bytes=$$(wc -c < quasar/target/deploy/merkle.so) && \
	anchor_bytes=$$(wc -c < anchor/target/deploy/anchor.so) && \
	pinocchio_bytes=$$(wc -c < pinocchio/target/deploy/merkle_pinocchio.so) && \
	quasar_sol=$$(solana rent $$quasar_bytes | awk '{print $$3}') && \
	anchor_sol=$$(solana rent $$anchor_bytes | awk '{print $$3}') && \
	pinocchio_sol=$$(solana rent $$pinocchio_bytes | awk '{print $$3}') && \
	quasar_kb=$$(echo "scale=1; $$quasar_bytes / 1024" | bc) && \
	anchor_kb=$$(echo "scale=1; $$anchor_bytes / 1024" | bc) && \
	pinocchio_kb=$$(echo "scale=1; $$pinocchio_bytes / 1024" | bc) && \
	echo "" && \
	printf "%-10s %-15s %-15s %-15s\n" "" "anchor" "quasar" "pinocchio" && \
	printf "%-10s %-15s %-15s %-15s\n" "binary" "$${anchor_kb}kb" "$${quasar_kb}kb" "$${pinocchio_kb}kb" && \
	printf "%-10s %-15s %-15s %-15s\n" "deploy" "$${anchor_sol} SOL" "$${quasar_sol} SOL" "$${pinocchio_sol} SOL" && \
	echo ""

cu:
	@echo "quasar"
	@cd quasar && quasar build > /dev/null 2>&1 && cargo test -- --nocapture 2>&1 | grep -E "initialize CU|insert CU|verify CU"
	@echo ""
	@echo "anchor"
	@cd anchor && anchor build > /dev/null 2>&1 && cargo test -- --nocapture 2>&1 | grep -E "initialize CU|insert CU|verify CU"
	@echo ""
	@echo "pinocchio"
	@cd pinocchio && cargo build-sbf > /dev/null 2>&1 && cargo test -- --nocapture 2>&1 | grep -E "initialize CU|insert CU|verify CU"
