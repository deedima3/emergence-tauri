import { z } from 'zod'

const loginSchema = z.object({
    password: z.string().nonempty("Password harus diisi!"),
  });

  const registerSchemaWithKey = z.object({
    encrtyption_key : z.string(),
    password: z.string().nonempty("Password harus diisi!"),
    confirm_password: z.string().nonempty("Password harus diisi!"),
  }).refine((data) => data.password === data.confirm_password, {
    message: "Password tidak sama",
    path: ["confirm_password"],
  });

  const registerSchemaWithoutKey = z.object({
    password: z.string().nonempty("Password harus diisi!"),
    confirm_password: z.string().nonempty("Password harus diisi!"),
  }).refine((data) => data.password === data.confirm_password, {
    message: "Password tidak sama",
    path: ["confirm_password"],
  });

  const registerSchema = z.union([registerSchemaWithKey, registerSchemaWithoutKey])

  const renameSchema = z.object({
    name : z.string().nonempty("Nama Folder harus diisi!"),
  })

  export {
    registerSchema, loginSchema, renameSchema
  }