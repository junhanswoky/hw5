namespace rs mini_redis
enum RequestType {
	Get,
	Set,
	Ping,
	Del,
}
struct RedisRequest {
	1: optional string key,
	2: optional string value,
	3: optional i32 extime,
	4: required RequestType req_type,
}
enum ResponseType {
	Output,
	Trap,
}
struct RedisResponse {
	1: required ResponseType resp_type,
	2: optional string value,
}
service RedisService {
	RedisResponse RedisCommand(1: RedisRequest req),
}