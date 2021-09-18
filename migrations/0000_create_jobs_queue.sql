create table job_queue(
  id binary(16) not null,
  message text not null,
  process_id varbinary(255) null,

  PRIMARY KEY (id),
  KEY process_id (process_id) 
);
