# Guia de Instalação do Arch Linux

Este guia fornece instruções detalhadas para a instalação do Arch Linux, incluindo configuração do teclado, conexão Wi-Fi, particionamento de disco, montagem de sistemas de arquivos e instalação de pacotes essenciais.

## 1. Configurar o Teclado

```bash
loadkeys br-abnt2
# Carrega o layout do teclado brasileiro ABNT2.
```

## 2. Conectar ao Wi-Fi

```bash
rfkill unblock all
# Desbloqueia todos os dispositivos de rádio (Wi-Fi, Bluetooth, etc.).

iwctl station list
# Lista todas as interfaces de estação Wi-Fi.

iwctl
# Inicia a ferramenta de linha de comando do iwd.

# Dentro do iwctl:
station list
# Lista todas as interfaces de estação Wi-Fi.

station wlan0 get-networks
# Lista todas as redes disponíveis para a interface wlan0.

station wlan0 connect nome-da-rede
# Conecta à rede especificada.
```

## 3. Atualizar o Relógio do Sistema

```bash
timedatectl set-ntp true
# Habilita a sincronização automática do tempo via NTP.
```

## 4. Particionar o Disco

```bash
fdisk /dev/nvme0n1
# Utilitário de partição de disco para o disco NVMe.

# Seta o teclado
loadkeys br-abnt2
# loadkeys: Comando para carregar o layout do teclado.
# br-abnt2: Especifica o layout do teclado brasileiro ABNT2.

# Conectar ao Wi-Fi
rfkill unblock all
# rfkill: Gerencia dispositivos de rádio (Wi-Fi, Bluetooth, etc.).
# unblock all: Desbloqueia todos os dispositivos de rádio.

iwctl station list
# iwctl: Ferramenta de linha de comando do iwd para gerenciar conexões Wi-Fi.
# station list: Lista todas as interfaces de estação Wi-Fi.

iwctl
# iwctl: Inicia a ferramenta de linha de comando do iwd.

# Dentro do iwctl
station list        
# station list: Lista todas as interfaces de estação Wi-Fi.

station wlan0 get-networks
# station wlan0 get-networks: Lista todas as redes disponíveis para a interface wlan0.

station wlan0 connect nome-da-rede
# station wlan0 connect nome-da-rede: Conecta à rede especificada.

# Atualizar o relógio do sistema
timedatectl set-ntp true
# timedatectl: Comando para gerenciar configurações de data e hora.
# set-ntp true: Habilita a sincronização automática do tempo via NTP (Network Time Protocol).

# Particionar o disco usando fdisk
fdisk /dev/nvme0n1
# fdisk: Utilitário de particionamento de disco.
# /dev/nvme0n1: Especifica o disco NVMe.

# Criar a tabela de partições:
# - /dev/nvme0n1p1: Partição EFI, 512 MB
# - /dev/nvme0n1p2: Partição Btrfs, restante do espaço

```

## 5. Formatar as Partições

```bash
mkfs.fat -F32 /dev/nvme0n1p1
# Cria um sistema de arquivos FAT32 na partição EFI.

mkfs.btrfs /dev/nvme0n1p2
# Cria um sistema de arquivos Btrfs na segunda partição.
```

## 6. Criar Subvolumes Btrfs

```bash
mount /dev/nvme0n1p2 /mnt
# Monta a partição Btrfs temporariamente.

btrfs su cr /mnt/@
# Cria um subvolume para root.

btrfs su cr /mnt/@home
# Cria um subvolume para home.

btrfs su cr /mnt/@snapshots
# Cria um subvolume para snapshots.

btrfs su cr /mnt/@var_log
# Cria um subvolume para /var/log.

btrfs su cr /mnt/@var_cache_pacman
# Cria um subvolume para /var/cache/pacman/pkg.

umount /mnt
# Desmonta a partição temporariamente.
```

## 7. Montar os Subvolumes

```bash
mount -o subvol=@ /dev/nvme0n1p2 /mnt
# Monta o subvolume root.

mkdir /mnt/{boot,home,.snapshots,var/log,var/cache/pacman/pkg}
# Cria os diretórios necessários.

mount /dev/nvme0n1p1 /mnt/boot
# Monta a partição EFI.

mount -o subvol=@home /dev/nvme0n1p2 /mnt/home
# Monta o subvolume home.

mount -o subvol=@snapshots /dev/nvme0n1p2 /mnt/.snapshots
# Monta o subvolume snapshots.

mount -o subvol=@var_log /dev/nvme0n1p2 /mnt/var/log
# Monta o subvolume var_log.

mount -o subvol=@var_cache_pacman /dev/nvme0n1p2 /mnt/var/cache/pacman/pkg
# Monta o subvolume var_cache_pacman.
```

## 8. Instalar o Sistema Base

```bash
pacstrap /mnt base linux linux-firmware base-devel vim dhcpcd
# Instala pacotes base no sistema montado.
```

## 9. Gerar o fstab

```bash
genfstab -U /mnt >> /mnt/etc/fstab
# Gera o arquivo fstab usando UUIDs.
cat /mnt/etc/fstab
# Verifica o conteúdo do arquivo fstab.
```

## 10. Entrar no Novo Sistema

```bash
arch-chroot /mnt
# Troca para o novo sistema instalado.
```

## 11. Configurar Timezone e Relógio

```bash
ln -sf /usr/share/zoneinfo/Region/City /etc/localtime
# Configura o timezone.

hwclock --systohc
# Define o relógio de hardware a partir do relógio do sistema.
```

## 12. Configurar Localização

```bash
vim /etc/locale.gen
# Abre o arquivo de configuração de locales no editor Vim.

locale-gen
# Gera os locales especificados.

echo KEYMAP=br-abnt2 > /etc/vconsole.conf
# Define o layout do teclado para o console virtual.
```

## 13. Configurar o Hostname

```bash
echo arch > /etc/hostname
# Define o hostname para "arch".

echo "127.0.0.1    localhost" >> /etc/hosts
echo "::1          localhost" >> /etc/hosts
echo "127.0.1.1    meu-hostname.localdomain meu-hostname" >> /etc/hosts
# Adiciona correspondências no arquivo /etc/hosts.
```

## 14. Definir Senha do Root

```bash
passwd
# Define a senha do usuário root.
```

## 15. Criar Usuário

```bash
useradd -m -g users -G wheel,video,audio,kvm -s /bin/bash arch
# Adiciona um novo usuário.

passwd arch
# Define a senha para o usuário "arch".
```

## 16. Instalar Pacotes Essenciais

```bash
pacman -Sy dosfstools os-prober mtools network-manager-applet networkmanager wpa_supplicant wireless_tools dialog sudo
# Instala pacotes essenciais.
```

## 17. Instalar e Configurar o Bootloader GRUB

```bash
pacman -S grub efibootmgr
# Instala o GRUB e o efibootmgr.

grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=arch --recheck
# Instala o GRUB no modo EFI.

grub-mkconfig -o /boot/grub/grub.cfg
# Gera o arquivo de configuração do GRUB.
```

## 18. Configurar zRAM para Swap

```bash
pacman -S zramswap
# Instala o zramswap.

systemctl enable zramswap.service
# Habilita o serviço de zramswap.
```

## 19. Sair do Chroot, Desmontar Partições e Reiniciar

```bash
exit
# Sai do chroot.

umount -R /mnt
# Desmonta recursivamente todas as partições montadas.

reboot
# Reinicia o sistema.
```

## 20. Configuração Pós-Instalação

```bash
systemctl enable NetworkManager
# Habilita o serviço de NetworkManager.

systemctl start NetworkManager
# Inicia o serviço de NetworkManager.

# (Opcional) Instalar ambiente gráfico, drivers e gerenciador de login.
# Exemplo: KDE Plasma, drivers NVIDIA/AMD, e SDDM.
```