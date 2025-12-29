--
-- Table structure for table `clusters`
--

CREATE TABLE `clusters` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL,
  `owner` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '',
  `accessinfo` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Dumping data for table `clusters`
--

INSERT INTO `clusters` VALUES
(1,'lenox','Lenovo','ssh lenox'),
(2,'tp-devel','Christoph Pospiech','no public access');

--
-- Table structure for table `codes`
--

CREATE TABLE `codes` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(32) NOT NULL,
  `version` varchar(32) NOT NULL,
  `www` varchar(256) NOT NULL DEFAULT 'https://www.google.de/',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `testcases`
--

CREATE TABLE `testcases` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `cid` int(11) NOT NULL,
  `name` varchar(32) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `cid` (`cid`),
  CONSTRAINT `testcases_ibfk_1` FOREIGN KEY (`cid`) REFERENCES `codes` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `projects`
--

CREATE TABLE `projects` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(32) NOT NULL,
  `comment` varchar(256) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Table structure for table `customer_cases`
--

CREATE TABLE `customer_cases` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `tcid` int(11) NOT NULL,
  `prid` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `tcid` (`tcid`),
  KEY `prid` (`prid`) USING BTREE,
  CONSTRAINT `customer_cases_ibfk_1` FOREIGN KEY (`tcid`) REFERENCES `testcases` (`id`) ON DELETE CASCADE,
  CONSTRAINT `customer_cases_ibfk_2` FOREIGN KEY (`prid`) REFERENCES `projects` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `people`
--

CREATE TABLE `people` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `title` varchar(12) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '' COMMENT 'Prof. Dr.',
  `first_name` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci DEFAULT NULL COMMENT 'Given name',
  `middle` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '' COMMENT 'Middle name',
  `surname` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL COMMENT 'surname',
  `affiliation` enum('Lenovo','IBM','Mellanox','Intel','BSC','NVIDIA') NOT NULL DEFAULT 'Lenovo' COMMENT 'employer',
  `email` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL,
  `phone` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '',
  `mobile` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=24 DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Dumping data for table `people`
--

INSERT INTO `people` VALUES
(1,'Dr.','Francois','','Thomas','IBM','ft@fr.ibm.com','+33683258855',''),
(2,'','Eric','','Michel','Lenovo','emichel@lenovo.com','+33-6-24-76-72-21',''),
(3,'','Kevin','','Dean','Lenovo','kdean@lenovo.com','',''),
(4,'Dr.','Holger','','Holthoff','Lenovo','hholthoff@lenovo.com','+49-7243-3436203','+49-171-97-80-340'),
(6,'Dr.','Christoph','','Pospiech','Lenovo','pospiech-HD@t-online.de','+49-351-86269826','+49-1511-910-4597'),
(7,'Dr.','Peter','','Mayes','Lenovo','pmayes@lenovo.com','+44-2392-562480',''),
(8,'','Luis','','Cebamanos','Lenovo','lcebamanos@lenovo.com','',''),
(9,'Dr.','Conor','','Elrick','Lenovo','celrick@lenovo.com','',''),
(10,'','Achim','','Boemelburg','Lenovo','retired','',''),
(11,'','Suga','','Sugavanam','Lenovo','retired','',''),
(12,'','Evgeny','','Rybin','Intel','evgeny.rybin@intel.com','',''),
(15,'','Olivier','','Lagrasse','Lenovo','olagrasse@lenovo.com','+33-6-24-76-77-64',''),
(18,'Dr.','Florian','','Merz','Lenovo','fmerz@lenovo.com','','+49-173-5256112'),
(19,'','Alan','','Gray','NVIDIA','alang@nvidia.com','',''),
(20,'','Mikhail','','Plotnikov','Intel','mikhail.plotnikov@intel.com','',''),
(21,'','Vladislav','','Plotnikov','Intel','vladislav.plotnikov@intel.com','',''),
(22,'','Michael','','Hennecke','Intel','michael.hennecke@intel.com','',''),
(23,'','Vladimir','','Gajinov','Lenovo','','','');

--
-- Table structure for table `runs`
--

CREATE TABLE `runs` (
  `rid` int(11) NOT NULL AUTO_INCREMENT,
  `ccid` int(11) NOT NULL COMMENT 'Foreign key from customer_cases',
  `pid` int(11) NOT NULL COMMENT 'Foreign key from people',
  `clid` int(11) NOT NULL DEFAULT 1 COMMENT 'Foreign key from clusters',
  `fsid` int(11) NOT NULL COMMENT 'foreign key from filesystems',
  `nodes` int(8) NOT NULL DEFAULT 1 COMMENT '# of nodes',
  `ht` tinyint(4) unsigned NOT NULL DEFAULT 1 COMMENT 'hyper threading',
  `turbo` tinyint(1) NOT NULL DEFAULT 1 COMMENT 'turbo/boost ON',
  `clock` varchar(32) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL DEFAULT '' COMMENT 'CPU speed in KHz',
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
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `appl_builtin_prof`
--

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
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `environ`
--

CREATE TABLE `environ` (
  `rid` int(11) NOT NULL DEFAULT 0,
  `k` varchar(64) NOT NULL,
  `value` varchar(8192) NOT NULL DEFAULT '',
  KEY `rid` (`rid`),
  CONSTRAINT `environ_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `files`
--

CREATE TABLE `files` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(512) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `fileops`
--

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
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `filesystems`
--

CREATE TABLE `filesystems` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `mount_point` varchar(256) NOT NULL,
  `fstype` varchar(32) NOT NULL COMMENT 'type of fs',
  `blocksize` int(11) NOT NULL COMMENT 'block size in KB',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Table structure for table `hpm_events`
--

CREATE TABLE `hpm_events` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT 'event id',
  `name` varchar(512) CHARACTER SET latin1 COLLATE latin1_swedish_ci NOT NULL COMMENT 'event name',
  `description` varchar(2048) CHARACTER SET latin1 COLLATE latin1_swedish_ci NOT NULL DEFAULT '' COMMENT 'short description',
  `type` varchar(64) CHARACTER SET latin1 COLLATE latin1_swedish_ci DEFAULT NULL COMMENT 'like PAPI prescribed, PAPI native, PAPI off_core',
  `comment` varchar(1024) NOT NULL DEFAULT '' COMMENT 'further information',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Table structure for table `hpm`
--

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
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `io`
--

CREATE TABLE `io` (
  `rid` int(11) NOT NULL DEFAULT 0,
  `tid` int(6) NOT NULL,
  `callname` varchar(32) NOT NULL DEFAULT '',
  `avgbytes` float NOT NULL DEFAULT 0,
  `calls` int(11) NOT NULL DEFAULT 0,
  `time` float NOT NULL DEFAULT 0,
  KEY `rid` (`rid`),
  CONSTRAINT `io_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `io_details`
--

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
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `prof_libs`
--

CREATE TABLE `prof_libs` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT 'primary key',
  `name` varchar(1024) NOT NULL COMMENT 'libraries containing the profiled routines',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Table structure for table `prof_names`
--

CREATE TABLE `prof_names` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT 'primary key',
  `lib_id` int(11) NOT NULL COMMENT 'foreign key relating to libraries',
  `name` varchar(1024) NOT NULL COMMENT 'name of the profiled routine',
  PRIMARY KEY (`id`),
  KEY `lib_id` (`lib_id`),
  CONSTRAINT `prof_names_ibfk_1` FOREIGN KEY (`lib_id`) REFERENCES `prof_libs` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Table structure for table `iprof`
--

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

--
-- Table structure for table `locations`
--

CREATE TABLE `locations` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `type` enum('nodes','nets','fs','chassis') NOT NULL COMMENT 'type of location',
  `clid` int(11) NOT NULL COMMENT 'cluster id',
  `name` varchar(32) NOT NULL COMMENT 'location name',
  PRIMARY KEY (`id`),
  KEY `clid` (`clid`),
  CONSTRAINT `locations_ibfk_1` FOREIGN KEY (`clid`) REFERENCES `clusters` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Table structure for table `mmm`
--

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
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `mpi_names`
--

CREATE TABLE `mpi_names` (
  `id` smallint(8) NOT NULL AUTO_INCREMENT,
  `name` varchar(64) NOT NULL COMMENT 'Name of the MPI call',
  `chapter` tinyint(6) DEFAULT NULL COMMENT 'Chapter in MPI standard',
  `type` enum('p2p','collective','MPI-IO','other','not_set','MPI_types','Communicators') NOT NULL DEFAULT 'not_set' COMMENT 'Type of MPI Call',
  PRIMARY KEY (`id`),
  UNIQUE KEY `mpi_names_inx` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=349 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `mpi_names`
--

INSERT INTO `mpi_names` VALUES
(1,'MPI_Send',3,'p2p'),
(2,'MPI_Recv',3,'p2p'),
(3,'MPI_Get_count',3,'p2p'),
(4,'MPI_Bsend',3,'p2p'),
(5,'MPI_Ssend',3,'p2p'),
(6,'MPI_Rsend',3,'p2p'),
(7,'MPI_Buffer_attach',3,'p2p'),
(8,'MPI_Buffer_detach',3,'p2p'),
(9,'MPI_Isend',3,'p2p'),
(10,'MPI_Ibsend',3,'p2p'),
(11,'MPI_Issend',3,'p2p'),
(12,'MPI_Irsend',3,'p2p'),
(13,'MPI_Irecv',3,'p2p'),
(14,'MPI_Wait',3,'p2p'),
(15,'MPI_Test',3,'p2p'),
(16,'MPI_Request_free',3,'p2p'),
(17,'MPI_Waitany',3,'p2p'),
(18,'MPI_Testany',3,'p2p'),
(19,'MPI_Waitall',3,'p2p'),
(20,'MPI_Testall',3,'p2p'),
(21,'MPI_Waitsome',3,'p2p'),
(22,'MPI_Testsome',3,'p2p'),
(23,'MPI_Request_get_status',3,'p2p'),
(24,'MPI_Iprobe',3,'p2p'),
(25,'MPI_Probe',3,'p2p'),
(26,'MPI_Improbe',3,'p2p'),
(27,'MPI_Mprobe',3,'p2p'),
(28,'MPI_Mrecv',3,'p2p'),
(29,'MPI_Imrecv',3,'p2p'),
(30,'MPI_Cancel',3,'p2p'),
(31,'MPI_Test_cancelled',3,'p2p'),
(32,'MPI_Send_init',3,'p2p'),
(33,'MPI_Bsend_init',3,'p2p'),
(34,'MPI_Ssend_init',3,'p2p'),
(35,'MPI_Rsend_init',3,'p2p'),
(36,'MPI_Recv_init',3,'p2p'),
(37,'MPI_Start',3,'p2p'),
(38,'MPI_Startall',3,'p2p'),
(39,'MPI_Sendrecv',3,'p2p'),
(40,'MPI_Sendrecv_replace',3,'p2p'),
(41,'MPI_Type_contiguous',4,'MPI_types'),
(42,'MPI_Type_vector',4,'MPI_types'),
(43,'MPI_Type_create_hvector',4,'MPI_types'),
(44,'MPI_Type_indexed',4,'MPI_types'),
(45,'MPI_Type_create_hindexed',4,'MPI_types'),
(46,'MPI_Type_create_indexed_block',4,'MPI_types'),
(47,'MPI_Type_create_hindexed_block',4,'MPI_types'),
(48,'MPI_Type_create_struct',4,'MPI_types'),
(49,'MPI_Type_create_subarray',4,'MPI_types'),
(50,'MPI_Type_create_darray',4,'MPI_types'),
(51,'MPI_Get_address',4,'MPI_types'),
(52,'MPI_Type_size',4,'MPI_types'),
(53,'MPI_Type_size_x',4,'MPI_types'),
(54,'MPI_Type_get_extent',4,'MPI_types'),
(55,'MPI_Type_get_extent_x',4,'MPI_types'),
(56,'MPI_Type_create_resized',4,'MPI_types'),
(57,'MPI_Type_get_true_extent',4,'MPI_types'),
(58,'MPI_Type_get_true_extent_x',4,'MPI_types'),
(59,'MPI_Type_commit',4,'MPI_types'),
(60,'MPI_Type_free',4,'MPI_types'),
(61,'MPI_Type_dup',4,'MPI_types'),
(62,'MPI_Get_elements',4,'MPI_types'),
(63,'MPI_Get_elements_x',4,'MPI_types'),
(64,'MPI_Type_get_envelope',4,'MPI_types'),
(65,'MPI_Type_get_contents',4,'MPI_types'),
(66,'MPI_Pack',4,'MPI_types'),
(67,'MPI_Unpack',4,'MPI_types'),
(68,'MPI_Pack_size',4,'MPI_types'),
(69,'MPI_Pack_external',4,'MPI_types'),
(70,'MPI_Unpack_external',4,'MPI_types'),
(71,'MPI_Pack_external_size',4,'MPI_types'),
(72,'MPI_Barrier',5,'collective'),
(73,'MPI_Bcast',5,'collective'),
(74,'MPI_Gather',5,'collective'),
(75,'MPI_Gatherv',5,'collective'),
(76,'MPI_Scatter',5,'collective'),
(77,'MPI_Scatterv',5,'collective'),
(78,'MPI_Allgather',5,'collective'),
(79,'MPI_Allgatherv',5,'collective'),
(80,'MPI_Alltoall',5,'collective'),
(81,'MPI_Alltoallv',5,'collective'),
(82,'MPI_Alltoallw',5,'collective'),
(83,'MPI_Reduce',5,'collective'),
(84,'MPI_Op_create',5,'collective'),
(85,'MPI_Op_free',5,'collective'),
(86,'MPI_Allreduce',5,'collective'),
(87,'MPI_Reduce_local',5,'collective'),
(88,'MPI_Op_commutative',5,'collective'),
(89,'MPI_Reduce_scatter_block',5,'collective'),
(90,'MPI_Reduce_scatter',5,'collective'),
(91,'MPI_Scan',5,'collective'),
(92,'MPI_Exscan',5,'collective'),
(93,'MPI_Ibarrier',5,'collective'),
(94,'MPI_Ibcast',5,'collective'),
(95,'MPI_Igather',5,'collective'),
(96,'MPI_Igatherv',5,'collective'),
(97,'MPI_Iscatter',5,'collective'),
(98,'MPI_Iscatterv',5,'collective'),
(99,'MPI_Iallgather',5,'collective'),
(100,'MPI_Iallgatherv',5,'collective'),
(101,'MPI_Ialltoall',5,'collective'),
(102,'MPI_Ialltoallv',5,'collective'),
(103,'MPI_Ialltoallw',5,'collective'),
(104,'MPI_Ireduce',5,'collective'),
(105,'MPI_Iallreduce',5,'collective'),
(106,'MPI_Ireduce_scatter_block',5,'collective'),
(107,'MPI_Ireduce_scatter',5,'collective'),
(108,'MPI_Iscan',5,'collective'),
(109,'MPI_Iexscan',5,'collective'),
(110,'MPI_Group_size',6,'Communicators'),
(111,'MPI_Group_rank',6,'Communicators'),
(112,'MPI_Group_translate_ranks',6,'Communicators'),
(113,'MPI_Group_compare',6,'Communicators'),
(114,'MPI_Comm_group',6,'Communicators'),
(115,'MPI_Group_union',6,'Communicators'),
(116,'MPI_Group_intersection',6,'Communicators'),
(117,'MPI_Group_difference',6,'Communicators'),
(118,'MPI_Group_incl',6,'Communicators'),
(119,'MPI_Group_excl',6,'Communicators'),
(120,'MPI_Group_range_incl',6,'Communicators'),
(121,'MPI_Group_range_excl',6,'Communicators'),
(122,'MPI_Group_free',6,'Communicators'),
(123,'MPI_Comm_size',6,'Communicators'),
(124,'MPI_Comm_rank',6,'Communicators'),
(125,'MPI_Comm_compare',6,'Communicators'),
(126,'MPI_Comm_dup',6,'Communicators'),
(127,'MPI_Comm_dup_with_info',6,'Communicators'),
(128,'MPI_Comm_idup',6,'Communicators'),
(129,'MPI_Comm_create',6,'Communicators'),
(130,'MPI_Comm_create_group',6,'Communicators'),
(131,'MPI_Comm_split',6,'Communicators'),
(132,'MPI_Comm_split_type',6,'Communicators'),
(133,'MPI_Comm_free',6,'Communicators'),
(134,'MPI_Comm_set_info',6,'Communicators'),
(135,'MPI_Comm_get_info',6,'Communicators'),
(136,'MPI_Comm_test_inter',6,'Communicators'),
(137,'MPI_Comm_remote_size',6,'Communicators'),
(138,'MPI_Comm_remote_group',6,'Communicators'),
(139,'MPI_Intercomm_create',6,'Communicators'),
(140,'MPI_Intercomm_merge',6,'Communicators'),
(141,'MPI_Comm_create_keyval',6,'Communicators'),
(142,'MPI_Comm_free_keyval',6,'Communicators'),
(143,'MPI_Comm_set_attr',6,'Communicators'),
(144,'MPI_Comm_get_attr',6,'Communicators'),
(145,'MPI_Comm_delete_attr',6,'Communicators'),
(146,'MPI_Win_create_keyval',6,'Communicators'),
(147,'MPI_Win_free_keyval',6,'Communicators'),
(148,'MPI_Win_set_attr',6,'Communicators'),
(149,'MPI_Win_get_attr',6,'Communicators'),
(150,'MPI_Win_delete_attr',6,'Communicators'),
(151,'MPI_Type_create_keyval',6,'Communicators'),
(152,'MPI_Type_free_keyval',6,'Communicators'),
(153,'MPI_Type_set_attr',6,'Communicators'),
(154,'MPI_Type_get_attr',6,'Communicators'),
(155,'MPI_Type_delete_attr',6,'Communicators'),
(156,'MPI_Comm_set_name',6,'Communicators'),
(157,'MPI_Comm_get_name',6,'Communicators'),
(158,'MPI_Type_set_name',6,'Communicators'),
(159,'MPI_Type_get_name',6,'Communicators'),
(160,'MPI_Win_set_name',6,'Communicators'),
(161,'MPI_Win_get_name',6,'Communicators'),
(162,'MPI_Cart_create',7,'not_set'),
(163,'MPI_Dims_create',7,'not_set'),
(164,'MPI_Graph_create',7,'not_set'),
(165,'MPI_Dist_graph_create_adjacent',7,'not_set'),
(166,'MPI_Dist_graph_create',7,'not_set'),
(167,'MPI_Topo_test',7,'not_set'),
(168,'MPI_Graphdims_get',7,'not_set'),
(169,'MPI_Graph_get',7,'not_set'),
(170,'MPI_Cartdim_get',7,'not_set'),
(171,'MPI_Cart_get',7,'not_set'),
(172,'MPI_Cart_rank',7,'not_set'),
(173,'MPI_Cart_coords',7,'not_set'),
(174,'MPI_Graph_neighbors_count',7,'not_set'),
(175,'MPI_Graph_neighbors',7,'not_set'),
(176,'MPI_Dist_graph_neighbors_count',7,'not_set'),
(177,'MPI_Dist_graph_neighbors',7,'not_set'),
(178,'MPI_Cart_shift',7,'not_set'),
(179,'MPI_Cart_sub',7,'not_set'),
(180,'MPI_Cart_map',7,'not_set'),
(181,'MPI_Graph_map',7,'not_set'),
(182,'MPI_Neighbor_allgather',7,'not_set'),
(183,'MPI_Neighbor_allgatherv',7,'not_set'),
(184,'MPI_Neighbor_alltoall',7,'not_set'),
(185,'MPI_Neighbor_alltoallv',7,'not_set'),
(186,'MPI_Neighbor_alltoallw',7,'not_set'),
(187,'MPI_Ineighbor_allgather',7,'not_set'),
(188,'MPI_Ineighbor_allgatherv',7,'not_set'),
(189,'MPI_Ineighbor_alltoall',7,'not_set'),
(190,'MPI_Ineighbor_alltoallv',7,'not_set'),
(191,'MPI_Ineighbor_alltoallw',7,'not_set'),
(192,'MPI_Get_version',8,'not_set'),
(193,'MPI_Get_library_version',8,'not_set'),
(194,'MPI_Get_processor_name',8,'not_set'),
(195,'MPI_Alloc_mem',8,'not_set'),
(196,'MPI_Free_mem',8,'not_set'),
(197,'MPI_Comm_create_errhandler',8,'not_set'),
(198,'MPI_Comm_set_errhandler',8,'not_set'),
(199,'MPI_Comm_get_errhandler',8,'not_set'),
(200,'MPI_Win_create_errhandler',8,'not_set'),
(201,'MPI_Win_set_errhandler',8,'not_set'),
(202,'MPI_Win_get_errhandler',8,'not_set'),
(203,'MPI_File_create_errhandler',8,'not_set'),
(204,'MPI_File_set_errhandler',8,'not_set'),
(205,'MPI_File_get_errhandler',8,'not_set'),
(206,'MPI_Errhandler_free',8,'not_set'),
(207,'MPI_Error_string',8,'not_set'),
(208,'MPI_Error_class',8,'not_set'),
(209,'MPI_Add_error_class',8,'not_set'),
(210,'MPI_Add_error_code',8,'not_set'),
(211,'MPI_Add_error_string',8,'not_set'),
(212,'MPI_Comm_call_errhandler',8,'not_set'),
(213,'MPI_Win_call_errhandler',8,'not_set'),
(214,'MPI_File_call_errhandler',8,'not_set'),
(215,'MPI_Initialized',8,'not_set'),
(216,'MPI_Abort',8,'not_set'),
(217,'MPI_Finalized',8,'not_set'),
(218,'MPI_Info_create',9,'not_set'),
(219,'MPI_Info_set',9,'not_set'),
(220,'MPI_Info_delete',9,'not_set'),
(221,'MPI_Info_get',9,'not_set'),
(222,'MPI_Info_get_valuelen',9,'not_set'),
(223,'MPI_Info_get_nkeys',9,'not_set'),
(224,'MPI_Info_get_nthkey',9,'not_set'),
(225,'MPI_Info_dup',9,'not_set'),
(226,'MPI_Info_free',9,'not_set'),
(227,'MPI_Comm_spawn',10,'not_set'),
(228,'MPI_Comm_get_parent',10,'not_set'),
(229,'MPI_Comm_spawn_multiple',10,'not_set'),
(230,'MPI_Open_port',10,'not_set'),
(231,'MPI_Close_port',10,'not_set'),
(232,'MPI_Comm_accept',10,'not_set'),
(233,'MPI_Comm_connect',10,'not_set'),
(234,'MPI_Publish_name',10,'not_set'),
(235,'MPI_Unpublish_name',10,'not_set'),
(236,'MPI_Lookup_name',10,'not_set'),
(237,'MPI_Comm_disconnect',10,'not_set'),
(238,'MPI_Comm_join',10,'not_set'),
(239,'MPI_Win_create',11,'not_set'),
(240,'MPI_Win_allocate',11,'not_set'),
(241,'MPI_Win_allocate_shared',11,'not_set'),
(242,'MPI_Win_shared_query',11,'not_set'),
(243,'MPI_Win_create_dynamic',11,'not_set'),
(244,'MPI_Win_attach',11,'not_set'),
(245,'MPI_Win_detach',11,'not_set'),
(246,'MPI_Win_free',11,'not_set'),
(247,'MPI_Win_get_group',11,'not_set'),
(248,'MPI_Win_set_info',11,'not_set'),
(249,'MPI_Win_get_info',11,'not_set'),
(250,'MPI_Put',11,'not_set'),
(251,'MPI_Get',11,'not_set'),
(252,'MPI_Accumulate',11,'not_set'),
(253,'MPI_Get_accumulate',11,'not_set'),
(254,'MPI_Fetch_and_op',11,'not_set'),
(255,'MPI_Compare_and_swap',11,'not_set'),
(256,'MPI_Rput',11,'not_set'),
(257,'MPI_Rget',11,'not_set'),
(258,'MPI_Raccumulate',11,'not_set'),
(259,'MPI_Rget_accumulate',11,'not_set'),
(260,'MPI_Win_fence',11,'not_set'),
(261,'MPI_Win_start',11,'not_set'),
(262,'MPI_Win_complete',11,'not_set'),
(263,'MPI_Win_post',11,'not_set'),
(264,'MPI_Win_wait',11,'not_set'),
(265,'MPI_Win_test',11,'not_set'),
(266,'MPI_Win_lock',11,'not_set'),
(267,'MPI_Win_lock_all',11,'not_set'),
(268,'MPI_Win_unlock',11,'not_set'),
(269,'MPI_Win_unlock_all',11,'not_set'),
(270,'MPI_Win_flush',11,'not_set'),
(271,'MPI_Win_flush_all',11,'not_set'),
(272,'MPI_Win_flush_local',11,'not_set'),
(273,'MPI_Win_flush_local_all',11,'not_set'),
(274,'MPI_Win_sync',11,'not_set'),
(275,'MPI_Grequest_start',12,'not_set'),
(276,'MPI_Grequest_complete',12,'not_set'),
(277,'MPI_Status_set_elements',12,'not_set'),
(278,'MPI_Status_set_elements_x',12,'not_set'),
(279,'MPI_Status_set_cancelled',12,'not_set'),
(280,'MPI_Query_thread',12,'not_set'),
(281,'MPI_Is_thread_main',12,'not_set'),
(282,'MPI_File_open',13,'MPI-IO'),
(283,'MPI_File_close',13,'MPI-IO'),
(284,'MPI_File_delete',13,'MPI-IO'),
(285,'MPI_File_set_size',13,'MPI-IO'),
(286,'MPI_File_preallocate',13,'MPI-IO'),
(287,'MPI_File_get_size',13,'MPI-IO'),
(288,'MPI_File_get_group',13,'MPI-IO'),
(289,'MPI_File_get_amode',13,'MPI-IO'),
(290,'MPI_File_set_info',13,'MPI-IO'),
(291,'MPI_File_get_info',13,'MPI-IO'),
(292,'MPI_File_set_view',13,'MPI-IO'),
(293,'MPI_File_get_view',13,'MPI-IO'),
(294,'MPI_File_read_at',13,'MPI-IO'),
(295,'MPI_File_read_at_all',13,'MPI-IO'),
(296,'MPI_File_write_at',13,'MPI-IO'),
(297,'MPI_File_write_at_all',13,'MPI-IO'),
(298,'MPI_File_iread_at',13,'MPI-IO'),
(299,'MPI_File_iwrite_at',13,'MPI-IO'),
(300,'MPI_File_read',13,'MPI-IO'),
(301,'MPI_File_read_all',13,'MPI-IO'),
(302,'MPI_File_write',13,'MPI-IO'),
(303,'MPI_File_write_all',13,'MPI-IO'),
(304,'MPI_File_iread',13,'MPI-IO'),
(305,'MPI_File_iwrite',13,'MPI-IO'),
(306,'MPI_File_seek',13,'MPI-IO'),
(307,'MPI_File_get_position',13,'MPI-IO'),
(308,'MPI_File_get_byte_offset',13,'MPI-IO'),
(309,'MPI_File_read_shared',13,'MPI-IO'),
(310,'MPI_File_write_shared',13,'MPI-IO'),
(311,'MPI_File_iread_shared',13,'MPI-IO'),
(312,'MPI_File_iwrite_shared',13,'MPI-IO'),
(313,'MPI_File_read_ordered',13,'MPI-IO'),
(314,'MPI_File_write_ordered',13,'MPI-IO'),
(315,'MPI_File_seek_shared',13,'MPI-IO'),
(316,'MPI_File_get_position_shared',13,'MPI-IO'),
(317,'MPI_File_read_at_all_begin',13,'MPI-IO'),
(318,'MPI_File_read_at_all_end',13,'MPI-IO'),
(319,'MPI_File_write_at_all_begin',13,'MPI-IO'),
(320,'MPI_File_write_at_all_end',13,'MPI-IO'),
(321,'MPI_File_read_all_begin',13,'MPI-IO'),
(322,'MPI_File_read_all_end',13,'MPI-IO'),
(323,'MPI_File_write_all_begin',13,'MPI-IO'),
(324,'MPI_File_write_all_end',13,'MPI-IO'),
(325,'MPI_File_read_ordered_begin',13,'MPI-IO'),
(326,'MPI_File_read_ordered_end',13,'MPI-IO'),
(327,'MPI_File_write_ordered_begin',13,'MPI-IO'),
(328,'MPI_File_write_ordered_end',13,'MPI-IO'),
(329,'MPI_File_get_type_extent',13,'MPI-IO'),
(330,'MPI_Register_datarep',13,'MPI-IO'),
(331,'MPI_File_set_atomicity',13,'MPI-IO'),
(332,'MPI_File_get_atomicity',13,'MPI-IO'),
(333,'MPI_File_sync',13,'MPI-IO'),
(334,'MPI_Keyval_create',15,'not_set'),
(335,'MPI_Keyval_free',15,'not_set'),
(336,'MPI_Attr_put',15,'not_set'),
(337,'MPI_Attr_get',15,'not_set'),
(338,'MPI_Attr_delete',15,'not_set'),
(339,'MPI_Type_create_f90_real',17,'not_set'),
(340,'MPI_Type_create_f90_complex',17,'not_set'),
(341,'MPI_Type_create_f90_integer',17,'not_set'),
(342,'MPI_Type_match_size',17,'not_set'),
(343,'MPI_Status_f2c',17,'not_set'),
(344,'MPI_Status_c2f',17,'not_set'),
(345,'MPI_Status_f082c',17,'not_set'),
(346,'MPI_Status_c2f08',17,'not_set'),
(347,'MPI_Status_f2f08',17,'not_set'),
(348,'MPI_Status_f082f',17,'not_set');

--
-- Table structure for table `mpi`
--

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
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `mpi_details`
--

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
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `power_types`
--

CREATE TABLE `power_types` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT 'power type id',
  `aggregation` enum('current','average','maximum','integral') NOT NULL DEFAULT 'current' COMMENT 'kind of aggregation',
  `unit` enum('W','KW','MW','J','KJ','MJ','KWh','C','MHz','GHz') NOT NULL DEFAULT 'W' COMMENT 'unit of power/energy value',
  `AC_DC` enum('AC','DC') NOT NULL DEFAULT 'DC' COMMENT 'AC or DC value',
  `tool` enum('IBMlib','ipmitool','RAPL','turbostat','ptumon','numactl','freeIPMI','other') NOT NULL DEFAULT 'RAPL' COMMENT 'tool that produced the value',
  `comment` varchar(128) NOT NULL DEFAULT '' COMMENT 'Additional comments',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=38 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `power_types`
--

INSERT INTO `power_types` VALUES
(1,'average','W','DC','IBMlib','old powerdc values from the energy table'),
(2,'average','W','AC','IBMlib','old powerac values from the energy table'),
(3,'integral','KWh','DC','IBMlib','old energydc values from the energy table'),
(4,'integral','KWh','AC','IBMlib','old energyac values from the energy table'),
(5,'current','W','DC','ipmitool','total node power from ipmitool'),
(6,'average','W','DC','ipmitool','average total node power for a run'),
(7,'current','W','DC','turbostat','turbostat PkgWatt total all packages'),
(8,'current','W','DC','turbostat','turbostat RAMWatt total all packages'),
(9,'current','W','DC','turbostat','turbostat PkgWatt package 0'),
(10,'current','W','DC','turbostat','turbostat RAMWatt package 0'),
(11,'current','W','DC','turbostat','turbostat PkgWatt package 1'),
(12,'current','W','DC','turbostat','turbostat RAMWatt package 1'),
(13,'current','W','AC','ipmitool','AC total node power from ipmitool'),
(14,'current','C','DC','turbostat','turbostat PkgTmp total all packages'),
(15,'current','C','DC','turbostat','turbostat PkgTmp package 0'),
(16,'current','C','DC','turbostat','turbostat PkgTmp package 1'),
(17,'current','MHz','DC','ptumon','ptumon CFreq CPU 0'),
(18,'current','C','DC','ptumon','ptumon Temp CPU 0'),
(19,'current','W','DC','ptumon','ptumon Power CPU 0'),
(20,'current','MHz','DC','ptumon','ptumon CFreq CPU 1'),
(21,'current','C','DC','ptumon','ptumon Temp CPU 1'),
(22,'current','W','DC','ptumon','ptumon Power CPU 1'),
(23,'current','C','DC','ptumon','ptumon Temp MEM 0'),
(24,'current','W','DC','ptumon','ptumon Power MEM 0'),
(25,'current','C','DC','ptumon','ptumon Temp MEM 1'),
(26,'current','W','DC','ptumon','ptumon Power MEM 1'),
(27,'average','W','DC','ipmitool','Read STARK energy register with raw ipmitool.'),
(28,'integral','KWh','DC','ipmitool','Read STARK energy register with raw ipmitool.'),
(29,'current','W','DC','ipmitool','ipmitool dcmi power reading'),
(30,'current','GHz','DC','numactl','clock frequency via numactl -C 2 perf stat -e task-clock,cycles sleep 2'),
(31,'integral','KJ','DC','ipmitool','Energy register for Lenovo AMD Genoa nodes (Kj).'),
(32,'integral','KWh','DC','ipmitool','Energy register for Lenovo AMD Genoa nodes (KWh).'),
(33,'current','W','DC','freeIPMI','SysPwr from Errics power measurement script'),
(34,'current','W','DC','freeIPMI','CPUPwr from Errics power measurement script'),
(35,'current','W','DC','freeIPMI','MemPwr from Errics power measurement script'),
(36,'current','C','DC','freeIPMI','CPU0Temp from Errics power measurement script'),
(37,'current','C','DC','freeIPMI','CPU1Temp from Errics power measurement script');

--
-- Table structure for table `power_aggregated`
--

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

--
-- Table structure for table `power_timeline`
--

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

--
-- Table structure for table `settings`
--

CREATE TABLE `settings` (
  `rid` int(11) NOT NULL,
  `k` varchar(64) NOT NULL COMMENT 'keyword',
  `value` varchar(8192) DEFAULT NULL,
  KEY `rid` (`rid`) USING BTREE,
  CONSTRAINT `settings_ibfk_1` FOREIGN KEY (`rid`) REFERENCES `runs` (`rid`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `tasks`
--

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
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

--
-- Table structure for table `userids`
--

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
) ENGINE=InnoDB AUTO_INCREMENT=75 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `userids`
--

INSERT INTO `userids` VALUES
(1,'emichel',1,2),
(2,'cp',2,6),
(3,'hholthoff',1,4),
(5,'xcpospiech',1,6),
(6,'pmayes',1,7),
(8,'lcebamanos',1,8),
(9,'celrick',1,9),
(12,'olagrasse',1,15),
(15,'fmerz',1,18);
