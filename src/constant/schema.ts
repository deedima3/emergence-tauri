import { z } from 'zod'

const loginSchema = z.object({
    password: z.string().nonempty("Password harus diisi!"),
  });

  const registerSchemaWithKey = z.object({
    encrtyption_key : z.string(),
    password: z.string().nonempty("Password harus diisi!"),
  });

  const registerSchemaWithoutKey = z.object({
    password: z.string().nonempty("Password harus diisi!"),
  });

  const registerSchema = z.union([registerSchemaWithKey, registerSchemaWithoutKey])

  export {
    registerSchema, loginSchema
  }