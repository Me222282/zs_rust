use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{ItemStruct, LitInt};
use quote::quote;
use crate::*;

pub(crate) fn gen_vector(attr: TokenStream, input: &mut ItemStruct) -> proc_macro2::TokenStream
{
    let li = syn::parse::<LitInt>(attr).expect("Expected a numerial size.");
    let size = li.base10_parse::<usize>().unwrap();
    
    let args = vector_args(size);
    let nums: Vec<_> = Numbers::new(size).collect();
    
    let unit_names = vector_args_str(size, "unit_");
    let uni_one = Ident::new("one", Span::call_site());
    let uni_zero = Ident::new("zero", Span::call_site());
    let units: Vec<_> = MatIdent::<Ident>::new(size, size, &uni_zero, &uni_one).collect();
    
    // let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let attrs = &input.attrs;
    let vis = &input.vis;
    
    // let selfRef = 
    
    return quote! {
        #(#attrs)*
        #[derive(Clone, Copy, Debug, PartialEq)]
        #vis struct #name<S>
        {
            #(pub #args: S),*
        }
        
        impl<S: Copy> #name<S>
        {
            #[inline]
            pub fn new(#(#args: S),*) -> Self
            {
                return Self { #(#args),* };
            }
            #[inline]
            pub fn single(value: S) -> Self
            {
                return Self { #(#args: value),* };
            }
        }
        impl<S: num_traits::One + num_traits::Zero + Copy> #name<S>
        {
            #(
            #[inline]
            pub fn #unit_names() -> Self
            {
                return Self::new(#(S::#units()),*);
            }
            )*
        }
        // impl<S: num_traits::NumCast> num_traits::NumCast for #name<S>
        // {
        //     #[inline]
        //     fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self>
        //     {
        //         let opt = S::from(n);
                
        //         return match opt
        //         {
        //             Some(v) => Some(Self
        //                 {
        //                     #(#args: v),*
        //                 })
        //             None => None
        //         }
        //     }
        // }
        impl<S: Copy> std::convert::From<S> for #name<S>
        {
            #[inline]
            fn from(value: S) -> Self
            {
                return Self::single(value);
            }
        }
        impl<S: Copy> std::convert::From<&[S; #li]> for #name<S>
        {
            #[inline]
            fn from(value: &[S; #li]) -> Self
            {
                return Self {
                    #(#args: value[#nums]),*
                };
            }
        }
        impl<S: Copy> std::convert::From<[S; #li]> for #name<S>
        {
            #[inline]
            fn from(value: [S; #li]) -> Self
            {
                return Self {
                    #(#args: value[#nums]),*
                };
            }
        }
        impl<S: Copy> std::convert::TryFrom<&[S]> for #name<S>
        {
            type Error = zs_core::SliceToVectorError;
            
            #[inline]
            fn try_from(value: &[S]) -> Result<Self, Self::Error>
            {
                if value.len() != #li
                {
                    return Err(zs_core::SliceToVectorError {});
                }
                
                return Ok(Self {
                    #(#args: value[#nums]),*
                });
            }
        }
        impl<S: Copy> std::convert::Into<[S; #li]> for #name<S>
        {
            #[inline]
            fn into(self) -> [S; #li]
            {
                return [
                    #(self.#args),*
                ];
            }
        }
        // impl<S: Copy> std::convert::Into<&[S]> for #name<S>
        // {
        //     #[inline]
        //     fn into(self) -> &[S]
        //     {
        //         return &[
        //             #(self.#args),*
        //         ];
        //     }
        // }
        
        impl<S: core::ops::Sub<Output = S> + core::ops::Add<Output = S> +
            core::ops::Mul<Output = S> + Sized + Copy> zs_core::VectorInt<S> for #name<S>
        {
            #[inline]
            fn dot(self, other: Self) -> S
            {
                return #(self.#args * other.#args)+* ;
            }
        }
        impl<S: num_traits::Zero + PartialEq> num_traits::Zero for #name<S>
        {
            #[inline]
            fn zero() -> Self
            {
                return Self
                {
                    #(#args: S::zero()),*
                };
            }
            #[inline]
            fn is_zero(&self) -> bool
            {
                return self == &Self::zero();
            }
        }
        impl<S: num_traits::One + PartialEq> num_traits::One for #name<S>
        {
            #[inline]
            fn one() -> Self
            {
                return Self
                {
                    #(#args: S::one()),*
                };
            }
            #[inline]
            fn is_one(&self) -> bool
            {
                return self == &Self::one();
            }
        }
        impl<S: core::ops::Add<Output = S>> core::ops::Add for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn add(self, rhs: Self) -> Self
            {
                return Self
                {
                    #(#args: self.#args + rhs.#args),*
                };
            }
        }
        impl<S: core::ops::Sub<Output = S>> core::ops::Sub for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn sub(self, rhs: Self) -> Self
            {
                return Self
                {
                    #(#args: self.#args - rhs.#args),*
                };
            }
        }
        impl<S: core::ops::Mul<Output = S>> core::ops::Mul for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn mul(self, rhs: Self) -> Self
            {
                return Self
                {
                    #(#args: self.#args * rhs.#args),*
                };
            }
        }
        impl<S: core::ops::Div<Output = S>> core::ops::Div for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn div(self, rhs: Self) -> Self
            {
                return Self
                {
                    #(#args: self.#args / rhs.#args),*
                };
            }
        }
        impl<S: core::ops::Rem<Output = S>> core::ops::Rem for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn rem(self, rhs: Self) -> Self
            {
                return Self
                {
                    #(#args: self.#args % rhs.#args),*
                };
            }
        }
        impl<S: core::ops::Add<Output = S> + Copy> core::ops::Add<S> for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn add(self, rhs: S) -> Self
            {
                return Self
                {
                    #(#args: self.#args + rhs),*
                };
            }
        }
        impl<S: core::ops::Sub<Output = S> + Copy> core::ops::Sub<S> for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn sub(self, rhs: S) -> Self
            {
                return Self
                {
                    #(#args: self.#args - rhs),*
                };
            }
        }
        impl<S: core::ops::Mul<Output = S> + Copy> core::ops::Mul<S> for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn mul(self, rhs: S) -> Self
            {
                return Self
                {
                    #(#args: self.#args * rhs),*
                };
            }
        }
        impl<S: core::ops::Div<Output = S> + num_traits::One + Copy> core::ops::Div<S> for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn div(self, rhs: S) -> Self
            {
                let div = S::one() / rhs;
                return Self
                {
                    #(#args: self.#args * div),*
                };
            }
        }
        impl<S: core::ops::Rem<Output = S> + Copy> core::ops::Rem<S> for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn rem(self, rhs: S) -> Self
            {
                return Self
                {
                    #(#args: self.#args % rhs),*
                };
            }
        }
        impl<S: core::ops::AddAssign> core::ops::AddAssign for #name<S>
        {
            #[inline]
            fn add_assign(&mut self, rhs: Self)
            {
                #(self.#args += rhs.#args);*
            }
        }
        impl<S: core::ops::SubAssign> core::ops::SubAssign for #name<S>
        {
            #[inline]
            fn sub_assign(&mut self, rhs: Self)
            {
                #(self.#args -= rhs.#args);*
            }
        }
        impl<S: core::ops::MulAssign> core::ops::MulAssign for #name<S>
        {
            #[inline]
            fn mul_assign(&mut self, rhs: Self)
            {
                #(self.#args *= rhs.#args);*
            }
        }
        impl<S: core::ops::DivAssign> core::ops::DivAssign for #name<S>
        {
            #[inline]
            fn div_assign(&mut self, rhs: Self)
            {
                #(self.#args /= rhs.#args);*
            }
        }
        impl<S: core::ops::RemAssign> core::ops::RemAssign for #name<S>
        {
            #[inline]
            fn rem_assign(&mut self, rhs: Self)
            {
                #(self.#args %= rhs.#args);*
            }
        }
        impl<S: core::ops::AddAssign + Copy> core::ops::AddAssign<S> for #name<S>
        {
            #[inline]
            fn add_assign(&mut self, rhs: S)
            {
                #(self.#args += rhs);*
            }
        }
        impl<S: core::ops::SubAssign + Copy> core::ops::SubAssign<S> for #name<S>
        {
            #[inline]
            fn sub_assign(&mut self, rhs: S)
            {
                #(self.#args -= rhs);*
            }
        }
        impl<S: core::ops::MulAssign + Copy> core::ops::MulAssign<S> for #name<S>
        {
            #[inline]
            fn mul_assign(&mut self, rhs: S)
            {
                #(self.#args *= rhs);*
            }
        }
        impl<S: core::ops::Div<Output = S> +
            core::ops::MulAssign + num_traits::One +
            Copy> core::ops::DivAssign<S> for #name<S>
        {
            #[inline]
            fn div_assign(&mut self, rhs: S)
            {
                let div = S::one() / rhs;
                #(self.#args *= div);*
            }
        }
        impl<S: core::ops::RemAssign + Copy> core::ops::RemAssign<S> for #name<S>
        {
            #[inline]
            fn rem_assign(&mut self, rhs: S)
            {
                #(self.#args %= rhs);*
            }
        }
        impl<S: core::ops::Neg<Output = S>> core::ops::Neg for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn neg(self) -> Self
            {
                return Self
                {
                    #(#args: -self.#args),*
                };
            }
        }
        
        impl<S: num_traits::Num> num_traits::Num for #name<S>
        {
            type FromStrRadixErr = &'static str;
            
            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr>
            {
                Err("string parsing not supported by vectors")
            }
        }
    };
}