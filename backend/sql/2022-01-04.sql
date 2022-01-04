/*
 Navicat Premium Data Transfer

 Source Server         : localhost
 Source Server Type    : MySQL
 Source Server Version : 80026
 Source Host           : localhost:3306
 Source Schema         : test

 Target Server Type    : MySQL
 Target Server Version : 80026
 File Encoding         : 65001

 Date: 04/01/2022 17:17:49
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for redis_info
-- ----------------------------
DROP TABLE IF EXISTS `redis_info`;
CREATE TABLE `redis_info`  (
  `id` int NOT NULL AUTO_INCREMENT,
  `host` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT 'redis的主机地址，可以是域名，也可以是ip',
  `port` smallint NOT NULL DEFAULT 6379 COMMENT 'redis的端口',
  `username` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '用户名（空表示无需用户名）',
  `password` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '密码（空表示无需密码）',
  `cluster_type` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT 'STANDALONE，CLUSTER，SENTINEL',
  `create_time` datetime NOT NULL DEFAULT '1900-01-01 00:00:00',
  `create_id` int NOT NULL DEFAULT -5,
  `update_time` datetime NOT NULL DEFAULT '1900-01-01 00:00:00',
  `update_id` int NOT NULL DEFAULT -5,
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT = 'redis信息主表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of redis_info
-- ----------------------------
INSERT INTO `redis_info` VALUES (1, 'localhost', 6379, '', '123456', 'STANDALONE', '1900-01-01 00:00:00', -5, '1900-01-01 00:00:00', -5);

-- ----------------------------
-- Table structure for redis_node_info
-- ----------------------------
DROP TABLE IF EXISTS `redis_node_info`;
CREATE TABLE `redis_node_info`  (
  `id` int NOT NULL,
  `redis_info_id` int NOT NULL COMMENT 'redis_info表的主键',
  `node_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT 'redis集群中，redis的唯一标志',
  `master_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT 'redis集群或哨兵模式中，当前node为从节点，此字段表示master的id，否则此字段为空',
  `host` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT 'redis的主机地址，可以是域名，也可以是ip',
  `port` mediumint NOT NULL DEFAULT 6379 COMMENT 'redis的端口',
  `node_role` varchar(8) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'MASTER(单节点就是MASTER),SLAVE',
  `node_status` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'CONNECTED,UNKNOWN,UNCONNECTED',
  `slot_from` mediumint NOT NULL DEFAULT 0 COMMENT '集群模式中，slot开始，非集群为0',
  `slot_to` mediumint NOT NULL DEFAULT 16383 COMMENT '集群模式中，slot结束，非集群为16383',
  `create_time` datetime NOT NULL DEFAULT '1900-01-01 00:00:00',
  `create_id` int NOT NULL DEFAULT -5,
  `update_time` datetime NOT NULL DEFAULT '1900-01-01 00:00:00',
  `update_id` int NOT NULL DEFAULT -5,
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT = 'redis节点信息表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of redis_node_info
-- ----------------------------

-- ----------------------------
-- Table structure for user
-- ----------------------------
DROP TABLE IF EXISTS `user`;
CREATE TABLE `user`  (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `password` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `create_time` datetime NOT NULL DEFAULT '1900-01-01 00:00:00',
  `create_id` int NOT NULL DEFAULT -5,
  `update_time` datetime NOT NULL DEFAULT '1900-01-01 00:00:00',
  `update_id` int NOT NULL DEFAULT -5,
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of user
-- ----------------------------
INSERT INTO `user` VALUES (1, 'admin', '123456', '1900-01-01 00:00:00', -5, '1900-01-01 00:00:00', -5);

SET FOREIGN_KEY_CHECKS = 1;
