archive:
	tar -czvf "HW/HW15_$$(date +"%Y-%m-%d_%H-%M").tar.gz" --exclude='target' --exclude='.git' --exclude='.idea' --exclude='.trunk' --exclude='HW' --same-permissions .

clean:
	cargo clean