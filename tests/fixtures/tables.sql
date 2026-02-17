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
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

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
  `affiliation` enum('Lenovo','IBM','retiree','Intel','BSC','NVIDIA','unknown') NOT NULL DEFAULT 'unknown' COMMENT 'employer',
  `email` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT 'not@known' COMMENT 'email address',
  `phone` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '',
  `mobile` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

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
  `tid` int(11) unsigned DEFAULT NULL,
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
  `tid` int(11) unsigned NOT NULL,
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
  `tid` int(11) unsigned NOT NULL COMMENT 'MPI rank',
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
  `tid` int(11) unsigned NOT NULL,
  `callname` varchar(32) NOT NULL DEFAULT '',
  `avgbytes` float NOT NULL DEFAULT 0,
  `calls` int(11) NOT NULL DEFAULT 0,
  `time` float NOT NULL DEFAULT 0,
  KEY `rid` (`rid`),
  CONSTRAINT `io_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `io_details` (
  `rid` int(11) NOT NULL DEFAULT 0,
  `tid` int(11) unsigned NOT NULL,
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
  `tid` int(11) unsigned NOT NULL COMMENT 'MPI rank',
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
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

CREATE TABLE `mpi` (
  `rid` int(11) NOT NULL DEFAULT 0,
  `tid` int(11) unsigned NOT NULL,
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
  `tid` int(11) unsigned NOT NULL,
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
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

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
  `tid` int(11) unsigned NOT NULL COMMENT 'MPI rank',
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
