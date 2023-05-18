

rbmq-start:
	cd docker && docker-compose up -d

rbmq-remove:
	cd docker && docker-compose down

rbmq-adduser:
	docker exec rabbitmq-dev bash -c "rabbitmqctl add_user dev devpass; rabbitmqctl add_vhost dev; rabbitmqctl set_user_tags dev dev; rabbitmqctl set_permissions -p dev dev ".*" ".*" ".*""

rbmq-purgeque:
	docker exec rabbitmq-dev bash -c "rabbitmqctl purge_queue --vhost=dev devque"

rbmq-deleteque:
	docker exec rabbitmq-dev bash -c "rabbitmqctl delete_queue --vhost=dev devque"

rbmq-into:
	docker exec -it rabbitmq-dev bash

rbmq-logs:
	cd docker && docker-compose logs -f --tail=10


