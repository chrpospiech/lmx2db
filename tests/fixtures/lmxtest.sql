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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `files`
--

CREATE TABLE `files` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(512) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;


--
-- Table structure for table `hpm`
--

DROP TABLE IF EXISTS `hpm`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;


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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
-- Table structure for table `prof_libs`
--

CREATE TABLE `prof_libs` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT 'primary key',
  `name` varchar(1024) NOT NULL COMMENT 'libraries containing the profiled routines',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `projects`
--

CREATE TABLE `projects` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(32) NOT NULL,
  `comment` varchar(256) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;

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
/*!40101 SET character_set_client = @saved_cs_client */;


LOCK TABLES `userids` WRITE;
/*!40000 ALTER TABLE `userids` DISABLE KEYS */;
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
/*!40000 ALTER TABLE `userids` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Dumping routines for database 'lmxtest'
--
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `cluster_id` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `cluster_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
CREATE DEFINER=`cpospiech`@`%` FUNCTION `cluster_name`(`clid` INT(11)) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Returns the cluster name for a given cluster id'
BEGIN
    DECLARE cluster_name VARCHAR(32);
    SELECT name INTO cluster_name FROM clusters WHERE id = clid;
    RETURN cluster_name;
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `code_id` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `code_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `code_version` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `customer_case_id` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `energy` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `environ_value` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `event_id` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `event_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
CREATE DEFINER=`cpospiech`@`%` FUNCTION `event_name`(`evid` INT(11)) RETURNS varchar(512) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve the event name from the event id or NULL'
BEGIN
    DECLARE ev_name VARCHAR(512);
    SELECT name INTO ev_name FROM hpm_events WHERE id = evid;
    RETURN ev_name;
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `filesystem_id` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `file_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `first_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `lib_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `lid_from_rid_tid` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
CREATE DEFINER=`cpospiech`@`%` FUNCTION `lid_from_rid_tid`(`runid` INT(11), `taskid` INT(11)) RETURNS int(11)
BEGIN
	DECLARE res int(11);
    SELECT lid INTO res FROM tasks
    WHERE rid = runid
    AND tid = taskid
    LIMIT 1;
    RETURN res;
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `location_cluster_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `location_id` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `location_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
CREATE DEFINER=`cpospiech`@`%` FUNCTION `location_name`(`loc_id` INT(11)) RETURNS varchar(32) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Return name of location identifier or NULL'
BEGIN
    DECLARE res VARCHAR(32);
    SELECT name INTO res FROM locations WHERE id = loc_id;
    RETURN res;
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `location_type` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
CREATE DEFINER=`cpospiech`@`%` FUNCTION `location_type`(`loc_id` INT(11)) RETURNS enum('nodes','nets','fs','chassis') CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    SQL SECURITY INVOKER
    COMMENT 'Teturn type of location or NULL,'
BEGIN
    DECLARE res ENUM('nodes', 'nets', 'fs', 'chassis');
    SELECT type INTO res FROM locations WHERE id = loc_id;
    RETURN res;
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `mpi_call_id` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `mpi_call_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
CREATE DEFINER=`cp`@`localhost` FUNCTION `mpi_call_name`(`m_id` SMALLINT(8)) RETURNS varchar(64) CHARSET utf8mb3 COLLATE utf8mb3_general_ci
    READS SQL DATA
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Retrieve name for mid'
BEGIN
    DECLARE m_name VARCHAR(64);
    SELECT name INTO m_name FROM mpi_names WHERE id = m_id;
    RETURN m_name;
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `person_id` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
       SELECT max(id)+1 INTO p_id FROM people;
       INSERT INTO people (id, name) VALUES (p_id, p_name);
    END IF;
    RETURN p_id;
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `person_id_for_uid` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `person_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `project_id` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `project_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `routine_id` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `routine_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `run_end_date` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `run_start_date` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `settings_value` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `submask2int` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
CREATE DEFINER=`cpospiech`@`%` FUNCTION `submask2int`(`mask_str` VARCHAR(128), `pos` INT(11), `len` INT(11)) RETURNS bigint(20) unsigned
    DETERMINISTIC
    SQL SECURITY INVOKER
    COMMENT 'Converts part of a binding masq into an integer'
BEGIN
    DECLARE res VARCHAR(256);
    SELECT CAST(CONV(SUBSTR(mask_str,pos,len),16,10) AS UNSIGNED)
    INTO res;
    RETURN res;
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `surname` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP FUNCTION IF EXISTS `testcase_id` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP FUNCTION IF EXISTS `testcase_name` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP PROCEDURE IF EXISTS `drop_pwr_timeline_by_rid` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'STRICT_TRANS_TABLES,ERROR_FOR_DIVISION_BY_ZERO,NO_AUTO_CREATE_USER,NO_ENGINE_SUBSTITUTION' */ ;
/*!50003 DROP PROCEDURE IF EXISTS `drop_run_by_user_start_date` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'NO_AUTO_VALUE_ON_ZERO' */ ;
/*!50003 DROP PROCEDURE IF EXISTS `get_file_byname` */;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_unicode_ci */ ;
DELIMITER ;;
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
END ;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;


-- Dump completed on 2025-12-28 20:33:32
