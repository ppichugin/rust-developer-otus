archive:
	tar -czvf "HW/HW01_$$(date +"%Y-%m-%d_%H-%M").tar.gz" --exclude='target' --exclude='.git' --exclude='.idea' --exclude='HW' --same-permissions .

clean:
	cargo clean