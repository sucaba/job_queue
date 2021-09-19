create table job_queue(
  order_pk int not null auto_increment,
  id binary(16) not null,
  status tinyint unsigned not null, -- 0 queued, 1 processing, 2 done, 3 failed
  message text not null,
  PRIMARY KEY (order_pk),
  KEY (id)
);
