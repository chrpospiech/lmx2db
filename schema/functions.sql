-- Copyright (c) 2026 C. Pospiech
--
-- This file is part of lmx2db.
--
-- Licensed under the Apache License, Version 2.0 (the "License");
-- you may not use this file except in compliance with the License.
-- You may obtain a copy of the License at
--
--     http://www.apache.org/licenses/LICENSE-2.0
--
-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS,
-- WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
-- See the License for the specific language governing permissions and
-- limitations under the License.

DELIMITER //
CREATE DEFINER=`cp`@`localhost` FUNCTION `cluster_id`(`cl_name` VARCHAR(32) CHARSET utf8mb3, `do_insert` INT(1)) RETURNS int(11)
    MODIFIES SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Returns the cluster id for a given cluster name'
BEGIN
    DECLARE cluster_id INT(11);
    SELECT id INTO cluster_id
              FROM clusters
              WHERE name LIKE cl_name
              LIMIT 1;
    IF (cluster_id IS NULL AND do_insert) THEN
       INSERT INTO clusters (name) VALUES (cl_name);
       SET cluster_id=last_insert_id();
    END IF;
    RETURN cluster_id;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `cluster_name`(`clid` INT(11)) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Returns the cluster name for a given cluster id'
BEGIN
    DECLARE cluster_name VARCHAR(32);
    SELECT name INTO cluster_name FROM clusters WHERE id = clid;
    RETURN cluster_name;
END;
//
--
DELIMITER //
CREATE DEFINER=`'cp'`@`'localhost'` FUNCTION `code_id`(`code_name` VARCHAR(32) CHARSET utf8mb3, `code_version` VARCHAR(32) CHARSET utf8mb3, `do_insert` INT(1)) RETURNS int(11)
    MODIFIES SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Get code version or insert data.'
BEGIN
    DECLARE c_id INT;
    SELECT id INTO c_id
              FROM codes
              WHERE name LIKE code_name
              AND version LIKE code_version
              LIMIT 1;
    IF (c_id IS NULL AND do_insert) THEN
       INSERT INTO codes (name, version) VALUES (code_name, code_version);
       SET c_id=last_insert_id();
    END IF;
    RETURN c_id;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `code_name`(`ccid` INT(11)) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Extract the code name from the customer case id'
BEGIN
    DECLARE c_name VARCHAR(32);
    SELECT t1.name INTO c_name
    FROM codes AS t1, testcases AS t2, customer_cases AS t3
    WHERE t3.id = ccid
    AND t2.id = t3.tcid
    AND t1.id = t2.cid;
    RETURN c_name;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `code_version`(`ccid` INT(11)) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Extract the code version from the customer case id'
BEGIN
    DECLARE c_version VARCHAR(32);
    SELECT t1.version INTO c_version
    FROM codes AS t1, testcases AS t2, customer_cases AS t3
    WHERE t3.id = ccid
    AND t2.id = t3.tcid
    AND t1.id = t2.cid
    LIMIT 1;
    RETURN c_version;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `customer_case_id`(`project` VARCHAR(32), `code` VARCHAR(32), `version` VARCHAR(32), `testcase` VARCHAR(32), `do_insert` INT(1)) RETURNS int(11)
    MODIFIES SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'retrieve customer case id for given proj,code, version, testcase'
BEGIN
    DECLARE cc_id INT;
    DECLARE p_id INT;
    DECLARE tc_id INT;
    SET p_id = project_id(project, do_insert);
    IF p_id IS NULL THEN
       RETURN p_id;
    END IF;
    SET tc_id = testcase_id(code, version, testcase, do_insert);
    IF tc_id IS NULL THEN
       RETURN tc_id;
    END IF;
    SELECT id INTO cc_id FROM customer_cases
       WHERE prid = p_id
       AND tcid = tc_id
       LIMIT 1;
    IF (cc_id IS NULL AND do_insert) THEN
       INSERT INTO customer_cases (prid, tcid)
           VALUES (p_id, tc_id);
        SET cc_id=last_insert_id();
    END IF;
    RETURN cc_id;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `energy`(rid_val int(11), type_val int(11)  ) RETURNS double
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Computes the energy from power_timeline values for some rid'
BEGIN
    DECLARE res DOUBLE;
    SELECT sum(value *
               (ptl1.timestamp + ptl1.timestamp_n*1.e-9 -
               (SELECT ptl2.timestamp + ptl2.timestamp_n*1.e-9
                AS last_time
                FROM `power_timeline` AS ptl2
                WHERE ptl2.timestamp + ptl2.timestamp_n*1.e-9
                    BETWEEN r.date_start + r.date_start_n*1.e-9
                    AND r.date_finish + r.date_finish_n*1.e-9
                AND ptl2.timestamp < ptl1.timestamp
                AND ptl2.type = ptl1.type
                AND ptl2.lid = ptl1.lid
                ORDER BY last_time DESC
                LIMIT 1))) INTO res
    FROM `power_timeline` AS ptl1,
    `runs` AS r, `run_locations` AS rl
    WHERE r.rid = rid_val
    AND ptl1.timestamp + ptl1.timestamp_n*1.e-9
        BETWEEN r.date_start + r.date_start_n*1.e-9
        AND r.date_finish + r.date_finish_n*1.e-9
    AND ptl1.type = type_val
    AND r.rid = rl.rid
    AND ptl1.lid = rl.lid
    LIMIT 1;
    RETURN res;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `environ_value`(`rid_par` INT(11), `k_par` VARCHAR(64) CHARSET latin1) RETURNS varchar(8192) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Extract value of environment variable from table environ.'
BEGIN
  DECLARE RET_VAL VARCHAR(8192);
  SELECT value into RET_VAL FROM environ
         WHERE rid = rid_par
         AND k = k_par;
  RETURN RET_VAL;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `event_id`(`ev_name` VARCHAR(512) CHARSET latin1, `do_insert` INT(1)) RETURNS int(11)
    READS SQL DATA
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the event id or NULL and potentially insert the event'
BEGIN
	DECLARE e_id INT;
    SELECT id INTO e_id FROM hpm_events
                        WHERE name LIKE ev_name LIMIT 1;
    IF (e_id IS NULL AND do_insert) THEN
       INSERT INTO hpm_events (name) VALUES (ev_name);
       SET e_id=last_insert_id();
    END IF;
    RETURN e_id;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `event_name`(`evid` INT(11)) RETURNS varchar(512) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the event name from the event id or NULL'
BEGIN
    DECLARE ev_name VARCHAR(512);
    SELECT name INTO ev_name FROM hpm_events WHERE id = evid;
    RETURN ev_name;
END;
//
--
DELIMITER //
CREATE DEFINER=`'cp'`@`'localhost'` FUNCTION `filesystem_id`(`p_fstype` VARCHAR(32) CHARSET utf8mb3, `p_mount_point` VARCHAR(256) CHARSET utf8mb3, `p_blocksize` INT(11)) RETURNS int(11)
    MODIFIES SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve fs_id or insert new filesystem.'
BEGIN
    DECLARE fs_id INT(11);
    SELECT id INTO fs_id FROM filesystems
       WHERE fstype LIKE p_fstype
       AND mount_point LIKE p_mount_point
       AND blocksize = p_blocksize
       LIMIT 1;
    IF (fs_id IS NULL) THEN
       INSERT INTO filesystems
           (fstype, mount_point, blocksize)
           VALUES
           (p_fstype, p_mount_point, p_blocksize);
       SET fs_id=last_insert_id();
    END IF;
    RETURN fs_id;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `file_name`(`file_id` INT(11)) RETURNS varchar(512) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the file name for a given id'
BEGIN
	DECLARE file_name VARCHAR(512);
    SELECT name INTO file_name
    			FROM files
    			WHERE id = file_id
                LIMIT 1;
    RETURN file_name;
END;
//
--
DELIMITER //
CREATE DEFINER=`'cp'`@`'localhost'` FUNCTION `first_name`(`full_name` VARCHAR(64) CHARSET utf8mb3) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the first_name from a full name'
BEGIN
    RETURN CASE LOCATE(' ', full_name)
        WHEN NULL THEN ""
        WHEN 0 THEN "%"
        ELSE SUBSTRING_INDEX(full_name, ' ', 1)
        END;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `lib_name`(`routine_id` INT(11)) RETURNS varchar(1024) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    READS SQL DATA
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the library name from the routine id or NULL'
BEGIN
    DECLARE lib_name VARCHAR(256);
    SELECT prof_libs.name INTO lib_name
    FROM prof_libs, prof_names
    WHERE prof_names.id = routine_id
    AND prof_libs.id = prof_names.lib_id
    LIMIT 1;
    RETURN lib_name;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `lid_from_rid_tid`(`runid` INT(11), `taskid` INT(11)) RETURNS int(11)
BEGIN
	DECLARE res int(11);
    SELECT lid INTO res FROM tasks
    WHERE rid = runid
    AND tid = taskid
    LIMIT 1;
    RETURN res;
END;
//
--
DELIMITER //
CREATE DEFINER=`'cp'`@`'localhost'` FUNCTION `location_cluster_name`(`loc_id` INT(11)) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve cluster name for given location identifier'
BEGIN
    DECLARE res INT(11);
    SELECT clid INTO res FROM locations WHERE id = loc_id LIMIT 1;
    IF res IS NULL THEN
       RETURN res;
    ELSE
       RETURN cluster_name(res);
    END IF;
END;
//
--
DELIMITER //
CREATE DEFINER=`'cp'`@`'localhost'` FUNCTION `location_id`(`loc_name` VARCHAR(32) CHARSET utf8mb3, `cl_name` VARCHAR(32) CHARSET utf8mb3, `loc_type` ENUM('nodes','nets','fs','chassis')) RETURNS int(11)
    MODIFIES SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Insert new location if not already there and return location id'
BEGIN
    DECLARE res INT(11);
    DECLARE cl_id INT(11);
    SET cl_id = cluster_id(cl_name, 0);
    SELECT id INTO res FROM locations
              WHERE name = loc_name
              AND clid = cl_id
              AND type = loc_type
              LIMIT 1;
    IF res IS NULL THEN
              INSERT INTO locations (name, clid, type)
              VALUES(loc_name, cl_id, loc_type);
              SET res=last_insert_id();
    END IF;
    RETURN (res);
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `location_name`(`loc_id` INT(11)) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Return name of location identifier or NULL'
BEGIN
    DECLARE res VARCHAR(32);
    SELECT name INTO res FROM locations WHERE id = loc_id;
    RETURN res;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `location_type`(`loc_id` INT(11)) RETURNS enum('nodes','nets','fs','chassis') CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    SQL SECURITY INVOKER
    COMMENT 'Teturn type of location or NULL,'
BEGIN
    DECLARE res ENUM('nodes', 'nets', 'fs', 'chassis');
    SELECT type INTO res FROM locations WHERE id = loc_id;
    RETURN res;
END;
//
--
DELIMITER //
CREATE DEFINER=`cp`@`localhost` FUNCTION `mpi_call_id`(`m_name` VARCHAR(64) CHARSET utf8mb3) RETURNS smallint(8)
    MODIFIES SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve id for MPI call'
BEGIN
    DECLARE m_id INT;
    SELECT id INTO m_id FROM mpi_names
                        WHERE name LIKE m_name
                        LIMIT 1;
    IF m_id IS NULL THEN
              INSERT INTO mpi_names (name)
              VALUES (m_name);
              SET m_id=last_insert_id();
    END IF;
    RETURN m_id;
END;
//
--
DELIMITER //
CREATE DEFINER=`cp`@`localhost` FUNCTION `mpi_call_name`(`m_id` SMALLINT(8)) RETURNS varchar(64) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve name for mid'
BEGIN
    DECLARE m_name VARCHAR(64);
    SELECT name INTO m_name FROM mpi_names WHERE id = m_id;
    RETURN m_name;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `person_id`(`p_name` VARCHAR(32), `do_insert` INT(1)) RETURNS int(11)
    MODIFIES SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the person_id or NULL and potentially insert the name'
BEGIN
    DECLARE p_id INT;
    SELECT id INTO p_id FROM people
       WHERE surname LIKE surname(p_name)
       AND first_name LIKE first_name(p_name)
       LIMIT 1;
    IF (p_id IS NULL AND do_insert) THEN
       INSERT INTO people (first_name, surname)
       VALUES (first_name(p_name), surname(p_name));
       SET p_id = LAST_INSERT_ID();
    END IF;
    RETURN p_id;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `person_id_for_uid`(`userid` VARCHAR(32) CHARSET utf8, `clusterid` INT(11)) RETURNS int(11)
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieves person id from userid on given cluster'
BEGIN
    DECLARE p_id INT;
    SELECT pid INTO p_id FROM userids
       WHERE name LIKE userid
       AND clid = clusterid
       LIMIT 1;
    RETURN p_id;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `person_name`(`pid` INT(11)) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the name from the person id or NULL'
BEGIN
    DECLARE p_name VARCHAR(64);
    SELECT CONCAT(first_name, ' ', surname)
    INTO p_name FROM people WHERE id = pid;
    RETURN p_name;
END;
//
--
DELIMITER //
CREATE DEFINER=`cp`@`localhost` FUNCTION `project_id`(`p_name` VARCHAR(32), `do_insert` INT(1)) RETURNS int(11)
    MODIFIES SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the project_id or NULL and potentially insert the name'
BEGIN
    DECLARE p_id INT;
    SELECT id INTO p_id
              FROM projects
              WHERE name LIKE p_name
              LIMIT 1;
    IF (p_id IS NULL AND do_insert) THEN
       INSERT INTO projects (name) VALUES (p_name);
       SET p_id=last_insert_id();
    END IF;
    RETURN p_id;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `project_name`(`ccid` INT(11)) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retieves the project name from the ccid'
BEGIN
    DECLARE p_name VARCHAR(32);
    SELECT t1.name INTO p_name
    FROM projects AS t1, customer_cases AS t2
    WHERE t2.id = ccid
    AND t1.id = t2.prid;
    RETURN p_name;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `routine_id`(`rout_name` VARCHAR(1024), `lib_name` VARCHAR(1024)) RETURNS int(11)
    MODIFIES SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'retrieve routine id for given routine, lib name'
BEGIN
    DECLARE l_id INT;
    DECLARE rt_id INT;
    SELECT id INTO l_id FROM prof_libs
       WHERE name LIKE lib_name
       LIMIT 1;
    IF (l_id IS NULL) THEN
       INSERT INTO prof_libs (name)
           VALUES (lib_name);
       SET l_id=last_insert_id();
    END IF;
    IF (l_id IS NULL) THEN
       return l_id;
    END IF;
    SELECT id INTO rt_id FROM prof_names
       WHERE name LIKE rout_name
       AND lib_id = l_id
       LIMIT 1;
    IF (rt_id IS NULL) THEN
       INSERT INTO prof_names (name, lib_id)
           VALUES (rout_name, l_id);
       SET rt_id=last_insert_id();
    END IF;
    RETURN rt_id;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `routine_name`(`routine_id` INT(11)) RETURNS varchar(1024) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    READS SQL DATA
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the routine name from the routine id or NULL'
BEGIN
    DECLARE r_name VARCHAR(256);
    SELECT name INTO r_name FROM prof_names
    	WHERE id = routine_id
        LIMIT 1;
    RETURN r_name;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `run_end_date`(`run_id` INT(11)) RETURNS double
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the date stamp for a run or return NULL'
BEGIN
    DECLARE res_date REAL;
    SELECT (date_finish + 1.e-9*date_finish_n) INTO res_date
    FROM runs WHERE rid = run_id;
    RETURN res_date;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `run_start_date`(`run_id` INT(11)) RETURNS double
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the date stamp for a run or return NULL'
BEGIN
    DECLARE res_date REAL;
    SELECT (date_start + 1.e-9*date_start_n) INTO res_date
    FROM runs WHERE rid = run_id;
    RETURN res_date;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `settings_value`(`rid_par` INT(11), `k_par` VARCHAR(64) CHARSET latin1) RETURNS varchar(8192) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Extract a value from the settings table or return NULL'
BEGIN
  DECLARE RET_VAL VARCHAR(8192);
  SELECT value into RET_VAL FROM settings
         WHERE rid = rid_par
         AND k = k_par;
  RETURN RET_VAL;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `submask2int`(`mask_str` VARCHAR(128), `pos` INT(11), `len` INT(11)) RETURNS bigint(20) unsigned
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Converts part of a binding masq into an integer'
BEGIN
    DECLARE res VARCHAR(256);
    SELECT CAST(CONV(SUBSTR(mask_str,pos,len),16,10) AS UNSIGNED)
    INTO res;
    RETURN res;
END;
//
--
DELIMITER //
CREATE DEFINER=`cp`@`localhost` FUNCTION `surname`(`full_name` VARCHAR(64) CHARSET utf8mb3) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the surname from a full name'
BEGIN
    DECLARE position INT(11);
    SET position = LOCATE(' ', REVERSE(full_name));
    RETURN CASE position
        WHEN NULL THEN ""
        WHEN 0 THEN RIGHT(full_name, 32)
        ELSE RIGHT(full_name, position - 1)
        END;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `testcase_id`(`code_name` VARCHAR(32), `code_version` VARCHAR(32), `tc_name` VARCHAR(32), `do_insert` INT(1)) RETURNS int(11)
    MODIFIES SQL DATA
    SQL SECURITY INVOKER
    COMMENT 'retrieve testcase id for given code, version, testcase name'
BEGIN
    DECLARE c_id INT;
    DECLARE tc_id INT;
    SET c_id = code_id(code_name, code_version, do_insert);
    IF (c_id IS NULL) THEN
       return c_id;
    END IF;
    SELECT id INTO tc_id FROM testcases
       WHERE name LIKE tc_name
       AND cid = c_id
       LIMIT 1;
    IF (tc_id IS NULL AND do_insert) THEN
       INSERT INTO testcases (name, cid)
           VALUES (tc_name, c_id);
       SET tc_id=last_insert_id();
    END IF;
    RETURN tc_id;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` FUNCTION `testcase_name`(`ccid` INT(11)) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Extract the testcase name from the customer case id'
BEGIN
    DECLARE tc_name VARCHAR(32);
    SELECT t1.name INTO tc_name
    FROM testcases AS t1, customer_cases AS t2
    WHERE t2.id = ccid
    AND t1.id = t2.tcid
    LIMIT 1;
    RETURN tc_name;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` PROCEDURE `drop_pwr_timeline_by_rid`(IN `rid_val` INT(11), IN `envelope` INT(11))
    MODIFIES SQL DATA
    SQL SECURITY INVOKER
    COMMENT 'drop power_timeline entries belonging to run rid_val'
BEGIN
    DELETE FROM power_timeline
        WHERE `lid` IN (SELECT lid
                    FROM run_locations
                    WHERE rid = rid_val)
    AND `timestamp`
      BETWEEN (   (SELECT date_start
                   FROM runs
                   WHERE rid = rid_val)
               +  1.e-9*(SELECT date_start_n
                         FROM runs
                         WHERE rid = rid_val)
               - envelope)
          AND     (  (SELECT date_finish
                  FROM runs
                  WHERE rid = rid_val)
               + 1.e-9*(SELECT date_finish_n
                        FROM runs
                        WHERE rid = rid_val)
               + envelope);
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` PROCEDURE `drop_run_by_user_start_date`(IN `pid_in` INT(11), IN `start_date_in` INT(11), IN `start_date_n_in` INT(11))
    MODIFIES SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Drop run with given pid and start_date and cascade deletes.'
BEGIN
    DELETE FROM runs
    WHERE pid = pid_in
    AND start_date = start_date_in
    AND start_date_n = start_date_n_in;
END;
//
--
DELIMITER //
CREATE DEFINER=`cpospiech`@`%` PROCEDURE `get_file_byname`(IN `N` VARCHAR(512), OUT `R` INT(11))
    MODIFIES SQL DATA
    SQL SECURITY INVOKER
BEGIN
    SET @n = N;
    SELECT id INTO R FROM files WHERE name like N;
    IF R IS NULL THEN
       INSERT INTO files(name) VALUES(N);
       SET R=last_insert_id();
    END IF;
END;
//
