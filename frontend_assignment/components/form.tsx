import React from "react";
import { useForm, SubmitHandler } from "react-hook-form";
import { yupResolver } from "@hookform/resolvers/yup";
import * as yup from "yup";

import styles from "../styles/Home.module.css";

type FormData = {
  name: string;
  age: number;
  address: string;
};

const schema = yup
  .object()
  .shape({
    name: yup.string().required(),
    age: yup.number().positive().integer().required(),
    address: yup.string().required(),
  })
  .required();

export default function App() {
  const {
    register,
    handleSubmit,
    formState: { errors },
    reset,
  } = useForm<FormData>({
    resolver: yupResolver(schema),
  });

  const onSubmit: SubmitHandler<FormData> = (data) => {
    console.log("data = ", { data });
    reset();
  };

  return (
    <div className={styles.container}>
      <main className={styles.main}>
        <form onSubmit={handleSubmit(onSubmit)}>
          name
          <input {...register("name")} />
          <p>{errors.name?.message}</p>
          age
          <input {...register("age")} />
          <p>{errors.age?.message}</p>
          address
          <input {...register("address")} />
          <p>{errors.address?.message}</p>
          <input type="submit" value="submit" />
        </form>
      </main>
    </div>
  );
}
