CREATE TABLE tb01 (
	id serial NOT NULL,
	col_texto text NULL,
	col_dt timestamp NOT NULL,
	CONSTRAINT tb01_pk PRIMARY KEY (id)
);
CREATE INDEX tb01_col_dt_idx ON tb01 (col_dt DESC);

