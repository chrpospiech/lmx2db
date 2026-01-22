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

CREATE TABLE `clusters` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL,
  `owner` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '',
  `accessinfo` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `locations` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `type` enum('nodes','nets','fs','chassis') NOT NULL COMMENT 'type of location',
  `clid` int(11) NOT NULL COMMENT 'cluster id',
  `name` varchar(32) NOT NULL COMMENT 'location name',
  PRIMARY KEY (`id`),
  KEY `clid` (`clid`),
  CONSTRAINT `locations_ibfk_1` FOREIGN KEY (`clid`) REFERENCES `clusters` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `codes` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(32) NOT NULL,
  `version` varchar(32) NOT NULL,
  `www` varchar(256) NOT NULL DEFAULT 'https://www.google.de/',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `testcases` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `cid` int(11) NOT NULL,
  `name` varchar(32) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `cid` (`cid`),
  CONSTRAINT `testcases_ibfk_1` FOREIGN KEY (`cid`) REFERENCES `codes` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `projects` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(32) NOT NULL,
  `comment` varchar(256) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `customer_cases` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `tcid` int(11) NOT NULL,
  `prid` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `tcid` (`tcid`),
  KEY `prid` (`prid`) USING BTREE,
  CONSTRAINT `customer_cases_ibfk_1` FOREIGN KEY (`tcid`) REFERENCES `testcases` (`id`) ON DELETE CASCADE,
  CONSTRAINT `customer_cases_ibfk_2` FOREIGN KEY (`prid`) REFERENCES `projects` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `people` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `title` varchar(12) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '' COMMENT 'Prof. Dr.',
  `first_name` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci DEFAULT NULL COMMENT 'Given name',
  `middle` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '' COMMENT 'Middle name',
  `surname` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL COMMENT 'surname',
  `affiliation` enum('Lenovo','IBM','retiree','Intel','BSC','NVIDIA') NOT NULL DEFAULT 'Lenovo' COMMENT 'employer',
  `email` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT 'not@known' COMMENT 'email address',
  `phone` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '',
  `mobile` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `userids` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT 'primary key',
  `name` varchar(32) NOT NULL COMMENT 'userId',
  `clid` int(11) NOT NULL COMMENT 'primary key of clusters',
  `pid` int(11) NOT NULL COMMENT 'primary key of person',
  PRIMARY KEY (`id`),
  KEY `clid` (`clid`),
  KEY `pid` (`pid`),
  CONSTRAINT `userids_ibfk_1` FOREIGN KEY (`clid`) REFERENCES `clusters` (`id`),
  CONSTRAINT `userids_ibfk_2` FOREIGN KEY (`pid`) REFERENCES `people` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `runs` (
  `rid` int(11) NOT NULL AUTO_INCREMENT,
  `ccid` int(11) NOT NULL COMMENT 'Foreign key from customer_cases',
  `pid` int(11) NOT NULL COMMENT 'Foreign key from people',
  `clid` int(11) NOT NULL DEFAULT 1 COMMENT 'Foreign key from clusters',
  `fsid` int(11) NOT NULL COMMENT 'foreign key from filesystems',
  `nodes` int(8) NOT NULL DEFAULT 1 COMMENT '# of nodes',
  `ht` tinyint(4) unsigned NOT NULL DEFAULT 1 COMMENT 'hyper threading',
  `clock` int(11) unsigned NOT NULL DEFAULT 0 COMMENT 'CPU speed in KHz (0 = turbo)',
  `validrun` tinyint(2) NOT NULL DEFAULT 0 COMMENT '0=unknown, -1=invalid, 1=valid',
  `has_MPItrace` tinyint(1) NOT NULL DEFAULT 0 COMMENT '1 if run has MPI traces',
  `has_iprof` tinyint(1) NOT NULL DEFAULT 0 COMMENT '1 if run has iprof',
  `start_date` int(11) NOT NULL COMMENT 'See LMX_summary',
  `start_date_n` int(11) NOT NULL COMMENT 'See LMX_summary',
  `stop_date` int(11) NOT NULL COMMENT 'See LMX_summary',
  `stop_date_n` int(11) NOT NULL COMMENT 'See LMX_summary',
  `MPI_ranks` int(8) unsigned NOT NULL DEFAULT 1 COMMENT 'See LMX_summary',
  `threads` int(6) unsigned NOT NULL DEFAULT 1 COMMENT 'threads per rank',
  `gpus` tinyint(6) unsigned NOT NULL DEFAULT 0 COMMENT 'Number of GPU cards used',
  `CUDA_version` varchar(16) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '' COMMENT 'CUDA version used for the run',
  `library_version` varchar(64) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL COMMENT 'lmx_trace version',
  `collect_scope` varchar(64) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL COMMENT 'lmx_trace scope',
  `collect_time` float NOT NULL DEFAULT 0 COMMENT 'lmx_trace collect time',
  `elapsed` float NOT NULL DEFAULT 0 COMMENT 'Elapsed time.',
  `perf_unit` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci DEFAULT NULL COMMENT 'unit name for performance value',
  `perf_value` float NOT NULL DEFAULT 0 COMMENT 'performance value in units',
  `md5sum_exe` binary(16) DEFAULT NULL COMMENT 'md5sum of executable',
  `git_commit` varchar(256) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci DEFAULT NULL COMMENT 'git commit of executable.',
  `dirname` varchar(256) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL,
  `comment` varchar(192) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '',
  `jobid` int(11) DEFAULT NULL,
  `mpilib` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL,
  `mpilib_version` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL,
  `compiler` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL,
  `compiler_version` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL,
  PRIMARY KEY (`rid`),
  KEY `ccid` (`ccid`),
  KEY `pid` (`pid`),
  KEY `runs_ibfk_3` (`clid`),
  CONSTRAINT `runs_ibfk_1` FOREIGN KEY (`ccid`) REFERENCES `customer_cases` (`id`) ON DELETE CASCADE,
  CONSTRAINT `runs_ibfk_2` FOREIGN KEY (`pid`) REFERENCES `people` (`id`) ON DELETE CASCADE,
  CONSTRAINT `runs_ibfk_3` FOREIGN KEY (`clid`) REFERENCES `clusters` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `appl_builtin_prof` (
  `rid` int(11) NOT NULL DEFAULT 0,
  `tid` int(6) DEFAULT NULL,
  `timestep` varchar(64) DEFAULT NULL COMMENT 'time step name or value',
  `routine` varchar(256) DEFAULT NULL COMMENT 'IMB/OMB: benchmark',
  `time` float DEFAULT NULL COMMENT '[s]',
  `avgtime` float DEFAULT NULL COMMENT '[us]',
  `mintime` float DEFAULT NULL COMMENT '[us]',
  `maxtime` float DEFAULT NULL COMMENT '[us]',
  `calls` int(8) DEFAULT NULL COMMENT 'IMB/OMB: repetitions',
  `bytes` int(11) DEFAULT NULL,
  `procs` int(8) DEFAULT NULL COMMENT 'IMB: # ranks',
  `bandwidth` float DEFAULT NULL COMMENT '[MBytes/s]',
  KEY `rid` (`rid`),
  CONSTRAINT `appl_builtin_prof_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `environ` (
  `rid` int(11) NOT NULL DEFAULT 0,
  `k` varchar(64) NOT NULL,
  `value` varchar(8192) NOT NULL DEFAULT '',
  KEY `rid` (`rid`),
  CONSTRAINT `environ_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `files` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(512) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `filesystems` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `mount_point` varchar(256) NOT NULL,
  `fstype` varchar(32) NOT NULL COMMENT 'type of fs',
  `blocksize` int(11) NOT NULL COMMENT 'block size in KB',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `fileops` (
  `rid` int(11) NOT NULL DEFAULT 0,
  `fid` int(11) NOT NULL DEFAULT 0,
  `tid` int(6) NOT NULL,
  `callname` varchar(32) NOT NULL,
  `avgbytes` float NOT NULL,
  `calls` int(11) NOT NULL,
  `time` float NOT NULL,
  KEY `rid` (`rid`),
  KEY `fid` (`fid`),
  CONSTRAINT `fileops_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE,
  CONSTRAINT `fileops_ibfk_2` FOREIGN KEY (`fid`) REFERENCES `files` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `hpm_events` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT 'event id',
  `name` varchar(512) CHARACTER SET latin1 COLLATE latin1_swedish_ci NOT NULL COMMENT 'event name',
  `description` varchar(2048) CHARACTER SET latin1 COLLATE latin1_swedish_ci NOT NULL DEFAULT '' COMMENT 'short description',
  `type` varchar(64) CHARACTER SET latin1 COLLATE latin1_swedish_ci DEFAULT NULL COMMENT 'like PAPI prescribed, PAPI native, PAPI off_core',
  `comment` varchar(1024) NOT NULL DEFAULT '' COMMENT 'further information',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `hpm` (
  `rid` int(11) NOT NULL COMMENT 'run id from table runs',
  `tid` int(6) NOT NULL COMMENT 'MPI rank',
  `regid` int(6) unsigned NOT NULL DEFAULT 0 COMMENT 'future use - currently 0',
  `evid` int(11) NOT NULL DEFAULT 0 COMMENT 'event id from hpm_events',
  `count` bigint(16) NOT NULL,
  KEY `rid` (`rid`),
  KEY `evid` (`evid`),
  CONSTRAINT `hpm_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE,
  CONSTRAINT `hpm_ibfk_2` FOREIGN KEY (`evid`) REFERENCES `hpm_events` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `io` (
  `rid` int(11) NOT NULL DEFAULT 0,
  `tid` int(6) NOT NULL,
  `callname` varchar(32) NOT NULL DEFAULT '',
  `avgbytes` float NOT NULL DEFAULT 0,
  `calls` int(11) NOT NULL DEFAULT 0,
  `time` float NOT NULL DEFAULT 0,
  KEY `rid` (`rid`),
  CONSTRAINT `io_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `io_details` (
  `rid` int(11) NOT NULL DEFAULT 0,
  `tid` int(6) NOT NULL,
  `callname` varchar(32) NOT NULL DEFAULT '',
  `avgbytes` float NOT NULL DEFAULT 0,
  `avgworldsize` float NOT NULL DEFAULT 0,
  `calls` int(11) NOT NULL DEFAULT 0,
  `time` float NOT NULL DEFAULT 0,
  KEY `rid` (`rid`),
  CONSTRAINT `io_details_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `prof_libs` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT 'primary key',
  `name` varchar(1024) NOT NULL COMMENT 'libraries containing the profiled routines',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `prof_names` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT 'primary key',
  `lib_id` int(11) NOT NULL COMMENT 'foreign key relating to libraries',
  `name` varchar(1024) NOT NULL COMMENT 'name of the profiled routine',
  PRIMARY KEY (`id`),
  KEY `lib_id` (`lib_id`),
  CONSTRAINT `prof_names_ibfk_1` FOREIGN KEY (`lib_id`) REFERENCES `prof_libs` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `iprof` (
  `rid` int(11) NOT NULL DEFAULT 0 COMMENT 'foreign key relating to table runs',
  `tid` int(6) NOT NULL COMMENT 'MPI rank',
  `thread_id` int(11) unsigned NOT NULL DEFAULT 0 COMMENT 'UNIX thread id or 0 indicating per process data',
  `routine_id` int(11) NOT NULL COMMENT 'foreign key relating to table prof_names',
  `ticks` int(11) unsigned NOT NULL DEFAULT 0 COMMENT 'interval timer ticks',
  KEY `rid` (`rid`),
  KEY `routine_id` (`routine_id`),
  CONSTRAINT `iprof_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE,
  CONSTRAINT `iprof_ibfk_2` FOREIGN KEY (`routine_id`) REFERENCES `prof_names` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `mmm` (
  `rid` int(11) NOT NULL,
  `mincomm` float DEFAULT NULL,
  `maxcomm` float DEFAULT NULL,
  `medcomm` float DEFAULT NULL,
  `mintask` int(6) DEFAULT NULL,
  `maxtask` int(6) DEFAULT NULL,
  `medtask` int(6) DEFAULT NULL,
  `minmpiio` float DEFAULT NULL,
  `maxmpiio` float DEFAULT NULL,
  `medmpiio` float DEFAULT NULL,
  `minmpiiotask` int(6) DEFAULT NULL,
  `maxmpiiotask` int(6) DEFAULT NULL,
  `medmpiiotask` int(6) DEFAULT NULL,
  `minloadimb` float DEFAULT NULL,
  `maxloadimb` float DEFAULT NULL,
  `medloadimb` float DEFAULT NULL,
  `minloadimbtask` int(6) DEFAULT NULL,
  `maxloadimbtask` int(6) DEFAULT NULL,
  `medloadimbtask` int(6) DEFAULT NULL,
  `minio` float DEFAULT NULL,
  `maxio` float DEFAULT NULL,
  `medio` float DEFAULT NULL,
  `miniotask` int(6) DEFAULT NULL,
  `maxiotask` int(6) DEFAULT NULL,
  `mediotask` int(6) DEFAULT NULL,
  KEY `rid` (`rid`),
  CONSTRAINT `mmm_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `mpi_names` (
  `id` smallint(8) NOT NULL AUTO_INCREMENT,
  `name` varchar(64) NOT NULL COMMENT 'Name of the MPI call',
  `chapter` tinyint(6) DEFAULT NULL COMMENT 'Chapter in MPI standard',
  `type` enum('p2p','collective','MPI-IO','other','not_set','MPI_types','Communicators') NOT NULL DEFAULT 'not_set' COMMENT 'Type of MPI Call',
  PRIMARY KEY (`id`),
  UNIQUE KEY `mpi_names_inx` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=349 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `mpi` (
  `rid` int(11) NOT NULL DEFAULT 0,
  `tid` int(6) NOT NULL,
  `mid` smallint(8) NOT NULL COMMENT 'Foreign key from mpi_names',
  `avgbytes` float NOT NULL DEFAULT 0,
  `calls` int(11) NOT NULL DEFAULT 0,
  `time` float NOT NULL DEFAULT 0,
  KEY `rid` (`rid`),
  KEY `mpi_ibfk_2` (`mid`),
  CONSTRAINT `mpi_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE,
  CONSTRAINT `mpi_ibfk_2` FOREIGN KEY (`mid`) REFERENCES `mpi_names` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `mpi_details` (
  `rid` int(11) NOT NULL DEFAULT 0,
  `tid` int(6) NOT NULL,
  `mid` smallint(8) NOT NULL COMMENT 'Foreign key from mpi_names',
  `avgbytes` float NOT NULL DEFAULT 0,
  `avgworldsize` float NOT NULL DEFAULT 0,
  `calls` int(11) NOT NULL DEFAULT 0,
  `time` float NOT NULL DEFAULT 0,
  KEY `rid` (`rid`),
  KEY `mpi_details_ibfk_2` (`mid`),
  CONSTRAINT `mpi_details_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE,
  CONSTRAINT `mpi_details_ibfk_2` FOREIGN KEY (`mid`) REFERENCES `mpi_names` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `power_types` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT 'power type id',
  `aggregation` enum('current','average','maximum','integral') NOT NULL DEFAULT 'current' COMMENT 'kind of aggregation',
  `unit` enum('W','KW','MW','J','KJ','MJ','KWh','C','MHz','GHz') NOT NULL DEFAULT 'W' COMMENT 'unit of power/energy value',
  `AC_DC` enum('AC','DC') NOT NULL DEFAULT 'DC' COMMENT 'AC or DC value',
  `tool` enum('IBMlib','ipmitool','RAPL','turbostat','ptumon','numactl','freeIPMI','other') NOT NULL DEFAULT 'RAPL' COMMENT 'tool that produced the value',
  `comment` varchar(128) NOT NULL DEFAULT '' COMMENT 'Additional comments',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=38 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `power_aggregated` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT 'primary key',
  `rid` int(11) NOT NULL DEFAULT 0 COMMENT 'foreign key relating to run id',
  `lid` int(11) NOT NULL DEFAULT 0 COMMENT 'foreign key location id',
  `type` int(11) unsigned NOT NULL DEFAULT 0 COMMENT 'foreign key relating to power_types',
  `value` float NOT NULL DEFAULT 0 COMMENT 'unit for the value defined by type',
  PRIMARY KEY (`id`),
  KEY `type` (`type`),
  KEY `rid` (`rid`),
  KEY `lid` (`lid`),
  CONSTRAINT `power_aggregated_ibfk_1` FOREIGN KEY (`type`) REFERENCES `power_types` (`id`) ON DELETE CASCADE,
  CONSTRAINT `power_aggregated_ibfk_2` FOREIGN KEY (`lid`) REFERENCES `locations` (`id`) ON DELETE CASCADE,
  CONSTRAINT `power_aggregated_ibfk_3` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `power_timeline` (
  `timestamp` int(11) unsigned NOT NULL DEFAULT 0 COMMENT 'Seconds since Jan 1, 1970 UTC',
  `timestamp_n` int(11) unsigned NOT NULL DEFAULT 0 COMMENT 'nanoseconds part of timestamp',
  `lid` int(11) NOT NULL DEFAULT 0 COMMENT 'foreign key location id',
  `type` int(11) unsigned NOT NULL DEFAULT 0 COMMENT 'foreign key relating to power_types',
  `value` float NOT NULL DEFAULT 0 COMMENT 'unit for the value defined by type',
  KEY `type` (`type`),
  KEY `lid` (`lid`),
  KEY `timestamp` (`timestamp`),
  CONSTRAINT `power_timeline_ibfk_1` FOREIGN KEY (`lid`) REFERENCES `locations` (`id`) ON DELETE CASCADE,
  CONSTRAINT `power_timeline_ibfk_2` FOREIGN KEY (`type`) REFERENCES `power_types` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `settings` (
  `rid` int(11) NOT NULL,
  `k` varchar(64) NOT NULL COMMENT 'keyword',
  `value` varchar(8192) DEFAULT NULL,
  KEY `rid` (`rid`) USING BTREE,
  CONSTRAINT `settings_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `tasks` (
  `rid` int(11) NOT NULL COMMENT 'run identifier',
  `tid` int(6) NOT NULL COMMENT 'MPI rank',
  `lid` int(11) NOT NULL COMMENT 'location index for node',
  `affinity` varbinary(4096) NOT NULL COMMENT 'from sched_getaffinity',
  `comm` float NOT NULL DEFAULT 0,
  `elapsed` float NOT NULL DEFAULT 0,
  `usertime` float NOT NULL DEFAULT 0,
  `systime` float NOT NULL DEFAULT 0,
  `mpiio` float NOT NULL DEFAULT 0,
  `io` float NOT NULL DEFAULT 0,
  `loadimb` float NOT NULL DEFAULT 0 COMMENT 'load imb time in s',
  `memory` float NOT NULL DEFAULT 0 COMMENT '[MiB]',
  `vmemory` float NOT NULL DEFAULT 0 COMMENT '[MiB]',
  KEY `rid` (`rid`),
  KEY `lid` (`lid`),
  CONSTRAINT `tasks_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE,
  CONSTRAINT `tasks_ibfk_2` FOREIGN KEY (`lid`) REFERENCES `locations` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

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

CREATE DEFINER=`cpospiech`@`%` FUNCTION `cluster_name`(`clid` INT(11)) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Returns the cluster name for a given cluster id'
BEGIN
    DECLARE cluster_name VARCHAR(32);
    SELECT name INTO cluster_name FROM clusters WHERE id = clid;
    RETURN cluster_name;
END;

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

CREATE DEFINER=`cpospiech`@`%` FUNCTION `lid_from_rid_tid`(`runid` INT(11), `taskid` INT(11)) RETURNS int(11)
BEGIN
	DECLARE res int(11);
    SELECT lid INTO res FROM tasks
    WHERE rid = runid
    AND tid = taskid
    LIMIT 1;
    RETURN res;
END;

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

CREATE DEFINER=`cpospiech`@`%` FUNCTION `location_name`(`loc_id` INT(11)) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Return name of location identifier or NULL'
BEGIN
    DECLARE res VARCHAR(32);
    SELECT name INTO res FROM locations WHERE id = loc_id;
    RETURN res;
END;

CREATE DEFINER=`cpospiech`@`%` FUNCTION `location_type`(`loc_id` INT(11)) RETURNS enum('nodes','nets','fs','chassis') CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    SQL SECURITY INVOKER
    COMMENT 'Teturn type of location or NULL,'
BEGIN
    DECLARE res ENUM('nodes', 'nets', 'fs', 'chassis');
    SELECT type INTO res FROM locations WHERE id = loc_id;
    RETURN res;
END;

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
