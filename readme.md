### 1. create .env file
```cp .env_example .env```
### 2. set .env variables
### 3. run server:
### 4. create table in and insert data in db:
```sql
CREATE TABLE `persons` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `persons` (`id`, `name`) VALUES (1, 'my name');
```
```cargo run```
### 4. open in browser
http://localhost:8081/persons/1