using DlclRpc;

namespace DlclNet.Models;

public struct Frame : IDTODerivable<FrameDTO>
{
    public List<Pixel> Pixels { get; set; }

    public FrameDTO ToDTO()
    {
        var pixels = Pixels.ConvertAll(pixel => pixel.ToDTO());
        var dto = new FrameDTO();
        dto.Pixels.AddRange(pixels);
        return dto;
    }
}