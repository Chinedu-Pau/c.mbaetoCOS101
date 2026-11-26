--
-- PostgreSQL database dump
--

\restrict pvaqfakewykGtxda6w6uVNhgvy8TLFvVPCWBe2AvjXV72D0IklkYoUQUG3ByiAA

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name character(50) NOT NULL,
    c_age integer,
    c_email character(50) NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email) FROM stdin;
110	Musta Karim                                       	35	m_karim@gmail.com                                 
111	Lilian Jaiya                                      	43	I_jaiye@gmail.com                                 
112	Aurthur Musa                                      	50	a_musa@gmail.com                                  
113	Philip Akonjo                                     	41	p_akonjo@gmail.com                                
114	Marylene Mapa                                     	33	m_mapa@gmail.com                                  
115	Oghenero Agor                                     	50	o_agor@gmail.com                                  
116	Adams Bree                                        	33	a_bree@gmail.com                                  
117	Okafor Mathias                                    	45	o_mathias@gmail.com                               
118	Samson Adeleke                                    	65	s_adeleke@gmail.com                               
119	Lawal Tamire                                      	35	l_tamire@gmail.com                                
120	James Job                                         	44	j_job@gmail.com                                   
121	Matthew Jakande                                   	21	m_jakande@gmail.com                               
122	Jimila Adegboye                                   	20	j_adegboye@gmail.com                              
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

\unrestrict pvaqfakewykGtxda6w6uVNhgvy8TLFvVPCWBe2AvjXV72D0IklkYoUQUG3ByiAA

