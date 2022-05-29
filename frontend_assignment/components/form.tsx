import React from "react";
import { useForm } from "react-hook-form";
import { yupResolver } from "@hookform/resolvers/yup";
import * as yup from "yup";

import styles from "../styles/Home.module.css";

interface IFormInputs {
  name: string;
  age: number;
  address: string;
}

const schema = yup
  .object({
    firstName: yup.string().required(),
    age: yup.number().positive().integer().required(),
    address: yup.string().required(),
  })
  .required();

export default function App() {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<IFormInputs>({
    resolver: yupResolver(schema),
  });
  const onSubmit = (data: IFormInputs) => console.log(data);

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
          <p>{errors.age?.message}</p>
          <input type="submit" value="submit" />
        </form>
      </main>
    </div>
  );
}
